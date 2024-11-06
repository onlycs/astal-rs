let notbound = (gir -o . -m not_bound) | lines

let normal = $notbound
| where ($it | str starts-with "[NOT GENERATED]")
| each { |it| $it | split row "]" | last | str trim }
| each { |it| $'"($it)"' }
| str join ", "

let builder = $notbound
| where ($it | str starts-with "[NOT GENERATED BUILDER]")
| each { |it| $it | split row "]" | last | str trim }
| each { |it| $it | split row "Builder" | first }
| each { |it|
    $'
    [[object]]
    name = "($it)"
    status = "generate"
    generate_builder = true
    '
}
| str join ""

let method = $notbound
| where ($it | str starts-with "[NOT GENERATED METHOD]")
| each { |it| $it | split row " because of " | last | str trim }
| each { |it| $it | split row " and " }
| flatten
| uniq
| each { |it| $'"($it)"' }
| str join ", "


def main [x: string] {
    match $x {
        "normal" => { echo $normal }
        "method" => { echo $method }
        "builder" => { echo $builder }
    }
}
