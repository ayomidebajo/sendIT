# sendIT
Send files to anyone using https

This project is built with Rust

Special thanks to @vishaltelangre for his project on searching files! I implemented his logic and remixed it for this project. Here is the link to the repo
https://github.com/vishaltelangre/ff and also on crates.io https://crates.io/crates/find-files

Videos on how to use this project coming soon!

## How to run this project

Download Ngrok into your machine

To receive open your terminal and run `cargo run` and then open another terminal from the root of your computer and run `ngrok http 8080` this will expose your receiever's port for two hours. The reciever's port is 8080

Next type `ifconfig` this command helps to tell you your machine's ip addr. Just look out for the Key `CSUM>` you'll see it beside "inet" (i.e "inet 198:991:900:00). Copy this or keep it somewhere. Then add the port number to it (i.e 198:991:900:00:8080). This is what the sender will use in the argument for the port address when sending.

To send, go back to the project directory, in another laptop to test out this functionality of sending to a different computer (you can also open another terminal instead). To run the sender's functionality type `cargo run FILENAME PORT_ADDRESS` 

Sender will be asked to input one more command "-c" to finally send the file.

The receiver should receive the file✨

Note the file search is only within your project directory!

Note also that if you send a file that the receiver already has, it'll just replace it.

This projects works well for Mac and Linux users, not so sure about windows PCs.

New features added

- If you edit the current file you sent and your terminal is still running, just type -c to resend the file, this will replace the old one you sent with the newly modified one!
- Devs can change the file search levels

## Support and Appreciation
If you found this project helpful, you could star my project. It goes a long way!

