.extern init_main_thread_rust

.text
.globl init_main_thread
.ent init_main_thread
init_main_thread:
        addi    $sp, -20
        sw      $t0, 16($sp)
        sw      $ra, 0($sp)

        jal     init_main_thread_rust

        lw      $ra, 0($sp)
        lw      $t0, 16($sp)
        addi    $sp, 20

        jr      $ra
.end init_main_thread
