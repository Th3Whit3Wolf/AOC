set shell := ["nu", "-c"]

todate := `date now | date to-table | get 0 | get year day | str join -`
day := `date now | date to-table | get 0 | get day`
year := `date now | date to-table | get 0 | get year`

alias r := run
run d1='' d2='' :
    #!/usr/bin/env nu
    let year = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d1}}' } else '{{year}}'
    let day = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d2}}' } else if ('{{d1}}' | str length) > 0 { '{{d1}}' } else '{{day}}'
    cargo run -p $'aoc-($year)-($day)'

alias t := test
test d1='' d2='' :
    #!/usr/bin/env nu
    let year = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d1}}' } else '{{year}}'
    let day = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d2}}' } else if ('{{d1}}' | str length) > 0 { '{{d1}}' } else '{{day}}'
    cargo test -p $'aoc-($year)-($day)'

alias i := init
alias create := init
alias new := init
alias gen := init
init d1='' d2='' :
    #!/usr/bin/env nu
    let year = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d1}}' } else '{{year}}'
    let day = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d2}}' } else if ('{{d1}}' | str length) > 0 { '{{d1}}' } else '{{day}}'
    let url = if ( $day | str starts-with '0') { $'https://adventofcode.com/($year)/day/($day | str replace '0' '')/input' } else {https://adventofcode.com/($year)/day/($day)/input }
    cargo generate --path {{justfile_directory()}}/template --name $'aoc-($year)-($day)' --destination ('{{justfile_directory()}}/' | path join $year )
    mv -v ('{{justfile_directory()}}/' | path join $year $'aoc-($year)-($day)') ('{{justfile_directory()}}/' | path join $year $'day-($day)')
    $env.AOC_COOKIE = (open $"{{justfile_directory()}}/AOC_COOKIE")
    http get --headers [Cookie session=($env.AOC_COOKIE)] $url | save ('{{justfile_directory()}}/' | path join $year $'day-($day)' input.txt)
    mut html = http get --headers [Cookie session=($env.AOC_COOKIE)] ($url | str replace '/input' '') 
    $html = ($html | str replace --all -r '<code>([\d \*=]+)</code>' '`$1`')
    $html = ($html | str replace --all -r '<code><em>(\d+)</em></code>' '**`$1`**')
    $html = ($html | str replace --all -r '<em>([^<]+)</em>' '**$1**')
    let link = "(" + ($"https://adventofcode.com/($year)/day/($day)") + ")"
    $html = ($html | str replace -r '<h2>--- (.+) ---</h2>' $'# [$1]($link)')
    $html = ($html | str replace -r '<h2 id="part2">--- (.+) ---</h2>' '## $1')
    $html = ($html | str replace --all '<pre><code>' '```' | str replace --all '</code></pre>' '```')
    $html = ($html | str replace --all -r '<p>(.*)</p>' '$1')
    $html = ($html | str replace --all -r '<a href="([^"]+)"[^>]*>([^<]*)</a>' '[$2]($1)')
    $html | query web --query '.day-desc' | to md --pretty | save ('{{justfile_directory()}}/' | path join $year $'day-($day)' ReadMe.md)


alias b := bench
bench d1='' d2='' :
    #!/usr/bin/env nu
    let year = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d1}}' } else '{{year}}'
    let day = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d2}}' } else if ('{{d1}}' | str length) > 0 { '{{d1}}' } else '{{day}}'
    cargo bench -p $'aoc-($year)-($day)'

alias b1 := bench1
bench1 d1='' d2='' :
    #!/usr/bin/env nu
    let year = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d1}}' } else '{{year}}'
    let day = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d2}}' } else if ('{{d1}}' | str length) > 0 { '{{d1}}' } else '{{day}}'
    cargo bench -p $'aoc-($year)-($day)' -- '1$'

alias b2 := bench2
bench2 d1='' d2='' :
    #!/usr/bin/env nu
    let year = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d1}}' } else '{{year}}'
    let day = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d2}}' } else if ('{{d1}}' | str length) > 0 { '{{d1}}' } else '{{day}}'
    cargo bench -p $'aoc-($year)-($day)' -- '2$'

bench-all:
    RUSTFLAGS="-C target-cpu=native" cargo bench -q | grep -v "running" | grep -v "test result" | grep -v "ii" | sed '/^$/N;/^\n$/D'  > benchmarks.txt

flamegraph d1='' d2='' part='1' :
    #!/usr/bin/env nu
    if ( '{{part}}' != '1' && '{{part}}' != '2' ) { error make {msg: "Part must be 1 or 2"} }
    let year = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d1}}' } else '{{year}}'
    let day = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d2}}' } else if ('{{d1}}' | str length) > 0 { '{{d1}}' } else '{{day}}'
    if not ('{{justfile_directory()}}/flamegraphs' | path join $year $'($day).svg' ) | path exists {
        mkdir -v ('{{justfile_directory()}}/flamegraphs' | path join $year $'($day).svg' )
    }
    cargo flamegraph --profile flamegraph --root --package $'aoc-($year)-($day)' --bin {{part}} -o $'aoc-($year)-($day)' -o ('{{justfile_directory()}}/flamegraphs' | path join $year $'($day).svg' )

dhat d1='' d2='' part='1':
    #!/usr/bin/env nu
    if ( '{{part}}' != '1' && '{{part}}' != '2' ) { error make {msg: "Part must be 1 or 2"} }
    let year = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d1}}' } else '{{year}}'
    let day = if ('{{d1}}' | str length) > 0 and ('{{d2}}' | str length)  > 0 { '{{d2}}' } else if ('{{d1}}' | str length) > 0 { '{{d1}}' } else '{{day}}'
    cargo run --profile dhat --features dhat-heap --package $'aoc-($year)-($day)' --bin {{part}} -o $'aoc-($year)-($day)' -o ('{{justfile_directory()}}/flamegraphs' | path join $year $'($day).svg' )


# You can find SESSION by using Chrome tools:
# 1) Go to https://adventofcode.com/2022/day/1/input
# 2) right-click -> inspect -> click the "Application" tab.
# 3) Refresh
# 5) Click https://adventofcode.com under "Cookies"
# 6) Grab the value for session. Fill it into your .env file
# 
# example .env:
#
# ```
# AOC_COOKIE='PASTE_COOKIE_VALUE_HERE'
# ```
#
# get the input for a day's puzzle
# get-input day:
#     ./scripts/get-aoc-input.rs --day {{day}} --current-working-directory {{justfile_directory()}}

get-input day lang='rust' year='2024':
    nu --plugins '[/opt/homebrew/bin/nu_plugin_query]' ./aoc.nu init {{lang}} {{day}} {{year}}