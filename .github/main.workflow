workflow "New workflow" {
  resolves = ["run"]
  on = "push"
}

action "run" {
  uses = "run"
}
