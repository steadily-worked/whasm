(module
  (func $log (import "imports" "log_func") (param i32))
  (func $how_old (param $year_now i32) (param $year_born i32) (result i32)
    local.get $year_now ;; put into stack
    local.get $year_born ;; put into stack
    i32.sub ;; just `sub` is enough. don't need to push again. result is on the top of the stack.
  )
  (func $log_how_old (param $year_now i32) (param $year_born i32)
    local.get $year_now
    local.get $year_born
    call $how_old
    call $log
  )

  (export "how_old" (func $how_old))
  (export "log_how_old" (func $log_how_old))
)