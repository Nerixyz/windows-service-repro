Reproduction example for [this stackoverflow question](https://stackoverflow.com/q/70533731/16300717).

# Instructions

## As a regular executable

* Run any application that emits SMTC events (for example a browser or **Spotify**).
* Start the application (`cargo run`).
* Toggle the play/pause in the app.
* Once an event is fired, the process should exit and `<time> Got Event` is written to stdout.
* A file, `test.log`, is created where the same message is written.

## As a service

I'm using `nssm` which runs the program as a service.
This is not exactly the same as making the executable a service, but it works for this reproduction.

* Download [nssm](https://nssm.cc/download) or use the provided executable:
    * Run `.\nssm.exe install <service-name>`
    * Set the `Path` to the binary created by cargo in `./target/debug`.
    * The working directory should be set automatically (this is where you will/would find the `test.log`).
    * Install the service.
* Make sure Spotify/your browser is still running (to provide events).
* Start the service:
    * **EITHER** Open the _Task Manager_, go to the _Services_ tab, find the service, right-click and click _Start_.
    * **OR** Run `.\nssm.exe start <service-name>` (needs admin rights)
* Toggle the playback
* **Expected**: a new line should be appended to `test.log`.
* Remove the service with `.\nssm remove <service-name>`.
