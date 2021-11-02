(module
 (export "fibonacci" (func $fibonacci))
 (func $fibonacci (; 0 ;) (param $n i32) (result i32)
  (if
   (i32.lt_s
    (get_local $n)
    (i32.const 2)
   )
   (return
    (i32.const 1)
   )
  )
  (return
   (i32.add
    (call $fibonacci
     (i32.sub
      (get_local $n)
      (i32.const 2)
     )
    )
    (call $fibonacci
     (i32.sub
      (get_local $n)
      (i32.const 1)
     )
    )
   )
  )
 )
)
