#ppl-service-rs

The simple example of installing service as protected process lite (PPL), usign ELAM driver.

###Repo structure
- ppl-service-rs - rust program to manage creation and deletion of PPL service
- elam-rs - a ELAM driver which allow store certificate. It's used to run service as PPL
- win-service-rs - windows service written in rust. It's the very simple service and do only one thing:
increase counter by 1 per second and log it. In addition, it delivers one important option. It's possible
to turn off service protection by sending there special control code

### Description
Anti-malware solutions typically use user-mode service that manages most actions. It's a 
single point of failure in case of attack. Therefore it should be protected extremely strong. 

To do that microsoft delivers PPL (protected process lite) mechanism. You can use ELAM (early lauch antimalware) driver 
to register your service as protected.

###Getting Started
1. At the beginning you need WDK and other stuff to build rust drivers. Let's look at: https://github.com/microsoft/windows-drivers-rs/
2. Next step is to install cargo make: https://github.com/sagiegurari/cargo-make
3. Use generate_cert.ps1 script to generate certificate necessary to signing, and resources to include by ELAM. Use:
`cargo make resources`
4. Build a binaries using: `cargo make compile`
5. Rename elam_rs.dll to elam_rs.sys: `cargo make rename`
6. At the end sign binaries. Use: `cargo make sign`

PS: You can use `cargo make` to invoke these free last steps

A signed binaries are stored in `target/debug` directory. It should be `ppl.exe`, `win-service.exe`, `elam_rs.sys`

###How to run
Place all binaries (ppl.exe, elam_rs.sys and win-service.exe) in the same folder on virtual machine. 

To create protected service type `ppl create_protected <service_name>`

Now you can inspect process and see it is protected.

At the end call `ppl delete_protected <service_name>` to remove service

IMPORTANT!!!
<br>Virtual machine must work in sign testing mode. To set this option run `bcdedit /set testsigning on` 
with administrator privileges and reboot your PC

###Links:
- https://learn.microsoft.com/en-us/windows/win32/services/protecting-anti-malware-services-
- https://github.com/microsoft/windows-drivers-rs/
- https://github.com/pathtofile/PPLRunner/tree/main/ppl_runner
