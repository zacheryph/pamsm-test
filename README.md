# pamsm test

Attempting to use [Crate](https://crates.io/crates/pamsm).
[Github Code](https://github.com/rcatolino/pam_sm_rust).

[Functions Not Showing Up](https://github.com/rcatolino/pam_sm_rust/blob/master/src/pam.rs#L89-L109)

shell output below. I can understand if not linking to pam cause it is
not calling anything from the pam library. Although I also do not know
if this is a requirement for PAM service modules even if it does not
use PAM functions.

I biggest issue is that I am not getting the `pam_sm_*` functions from
the `pamsm::pam` module. This in turn means the example cannot work
when pam tries to load it.

Attempting to replicate on smaller scale by moving the callback macro
to my `src/lib.rs` does not work and the functions show up.

I know `pamsm` hasn't been touched in quite a while. I am new to rust
and I can only assume that maybe this was a change made between rust
versions from then to now :x. Really though I have not a single clue.

```shell
$ rm -rf target                                                                    
$ cargo build
   Compiling libc v0.2.42
   Compiling pamsm-test v0.1.0 (file:///home/context/src/hive/pamsm-test)
   Compiling time v0.1.40
   Compiling pamsm v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.89 secs

$ ldd target/debug/libpamsm_test.so 
        linux-vdso.so.1 (0x00007ffeaef8f000)
        libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007f4decb91000)
        librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007f4dec989000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f4dec76c000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f4dec555000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f4dec1b6000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f4decfc3000)

$ strings target/debug/libpamsm_test.so|grep pam_sm
get_pam_sm
get_pam_sm
get_pam_sm
get_pam_sm

$
```

## Discovery

Calling a method such as `pamh.get_authtok` forces all `pam_sm_*` methods
to show up. So even though the `pam` module is being loaded it does not
include the callbacks unless we forcibly call something within `pam_raw`?

Below is the shared object file after adding the call to `pam_cached_authtok`

```shell
$ cargo build                                                                                        
   Compiling pamsm-test v0.1.0 (file:///home/context/src/hive/pamsm-test)                                                 
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs                                                         

$ ldd target/debug/libpamsm_test.so                                                                  
        linux-vdso.so.1 (0x00007ffd421e1000)                                                                              
        libpam.so.0 => /lib/x86_64-linux-gnu/libpam.so.0 (0x00007f34a9011000)                                             
        libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007f34a8e0d000)                                               
        librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007f34a8c05000)                                               
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f34a89e8000)                                     
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f34a87d1000)                                         
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f34a8432000)                                                 
        /lib64/ld-linux-x86-64.so.2 (0x00007f34a9458000)                                                                  
        libaudit.so.1 => /lib/x86_64-linux-gnu/libaudit.so.1 (0x00007f34a820a000)                                         
        libcap-ng.so.0 => /lib/x86_64-linux-gnu/libcap-ng.so.0 (0x00007f34a8004000)                                       

$ strings target/debug/libpamsm_test.so|grep pam_sm                                                  
➜  pamsm-test master ✗ strings target/debug/libpamsm_test.so|grep pam_sm_                                                 
pam_sm_acct_mgmt                                                                                                          
pam_sm_authenticate                                                                                                       
pam_sm_chauthtok                                                                                                          
pam_sm_close_session                                                                                                      
pam_sm_open_session                                                                                                       
pam_sm_setcred                                                                                                            

...[REDACTED]...
```
