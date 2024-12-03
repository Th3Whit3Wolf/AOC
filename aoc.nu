# plugin add /opt/homebrew/bin/nu_plugin_query
#plugin use query

#$env.AOC_COOKIE = (open $"($env.FILE_PWD)/.env" | from toml)
let today = (date now | date to-table | get 0)


def main [] {}

def "main init" [lang?: string, day?:int, year?:int] {
    open $"($env.FILE_PWD)/.env" | from toml | load-env
    let day = if ($day == null) {$today | get day} else if $day < 10 { $"0($day)" } else $day | into string
    let year = if ($year == null) {$today | get year} else if $year < 100 { $"20($year)" } else $year | into string
    let lang = if ($lang == null) {"rust"} else $lang 

    # Create day directory
    let out_dir = $"($env.FILE_PWD)/($year)/($lang)/day-($day)"
    mkdir -v $out_dir

    # Create Description.md
    let html_day = fetch_day $day $year
    create_description $html_day ($"($out_dir)/TASKS.md") $day $year

    # Copy template
    copy_template $out_dir

    # Save examples and solutions
    #create_examples_with_solutions $html_day $out_dir

    create_input $day $year $out_dir

    replace_template_strings ($"($out_dir)/Cargo.toml") $day $year
    replace_template_strings ($"($out_dir)/benches/benchmarks.rs") $day $year
    replace_template_strings ($"($out_dir)/src/part1.rs") $day $year
    replace_template_strings ($"($out_dir)/src/part2.rs") $day $year

}

def "main update" [lang?: string, day?:int, year?:int] {
    let day = if ($day == null) {$today | get day} else if $day < 10 { $"0($day)" } else $day
    let year = if ($year == null) {$today | get year} else if $year < 100 { $"20($year)" } else $year
    let lang = if ($lang == null) {"rust"} else $lang 

    # Create day directory
    let out_dir = $"($env.FILE_PWD)/($year)/($lang)/day-($day)"

    # Create Description.md
    let html_day = fetch_day $day $year
    create_description $html_day ($"($out_dir)/TASKS.md") $day $year

    # Save examples and solutions
    #create_examples_with_solutions $html_day $out_dir
}

def replace_template_strings [file: string, day: string, year: string] {
    echo $file
    let content = open $file --raw
    let content = ($content | str replace --all '{{package_name}}' $'aoc-($year)-($day)')
    let content = ($content | str replace --all '{{crate_name}}' $'aoc_($year)_($day)')
    let content = ($content | str replace --all '{{year}}' $'($year)')
    let content = ($content | str replace --all '{{day}}' $'($day)')
    $content | save $file -f
}

def create_input [day: string, year: string, out_dir: string] {
    let input = fetch_input $day $year
    $input | save ($"($out_dir)/input.txt") -f
}

def create_description [html: string, dest: string, day: string, year: string] {
    echo $html
    echo $dest 
    echo $day
    echo $year
    day_html_to_markdown $html $day $year | query web --query '.day-desc' | save $dest -f
}

def copy_template [dest: string] {
    cp -r template/* $dest
}

def create_examples_with_solutions [html: string, project_dir: string] {
    let examples_dir = ($"($project_dir)/examples")
    mkdir -v $examples_dir

    let examples = $html | query web -q '.day-desc pre'
    let solutions = $html | query web -q '.day-desc code em'
    for $i in 0..(($examples | length) - 1) {
        let example = $examples | get $i
        let solution = $solutions | reverse | get 0

        let ex_path = ($"($examples_dir)/($i).txt")
        if ($ex_path | path exists ) {} else {
            $example | save $ex_path
        }

        let sol_path = ($"($examples_dir)/($i)_solution.txt")
        if ($sol_path | path exists ) {} else {
            $solution | save $sol_path
        }
    }
}


def fetch_day [day: string, year: string] {
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day | into int)
}

def fetch_input [day: string, year: string] {
    http get --headers [Cookie session=($env.AOC_COOKIE)] https://adventofcode.com/($year)/day/($day | into int)/input
}

def day_html_to_markdown [html: string, day: string, year: string] {
    mut html = $html
    $html = ($html | str replace --all -r '<code>([\d \*=]+)</code>' '`$1`')
    $html = ($html | str replace --all -r '<code><em>(\d+)</em></code>' '**`$1`**')
    $html = ($html | str replace --all -r '<em>([^<]+)</em>' '**$1**')
    let link = "(" + ($"https://adventofcode.com/($year)/day/($day | into int)") + ")"
    $html = ($html | str replace -r '<h2>--- (.+) ---</h2>' $'# [$1]($link)
')
    $html = ($html | str replace -r '<h2 id="part2">--- (.+) ---</h2>' '## $1
')
    $html = ($html | str replace --all '<pre><code>' '```' | str replace --all '</code></pre>' '```')
    $html = ($html | str replace --all -r '<p>(.*)</p>' '$1
')
    $html = ($html | str replace --all -r '<a href="([^"]+)"[^>]*>([^<]*)</a>' '[$2]($1)')
    $html
}
