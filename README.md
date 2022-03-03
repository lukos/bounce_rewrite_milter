# bounce_rewrite_milter
A postfix milter for rewriting the Return-Path header

## What is it for?
When you are bulk sending email, it is common to want to use VERP to set a distinct return-path for each message so that bounces can be tracked back to the specific problem message. However,
the functionality built-in to postfix is a bit too basic and is essentially tied only to the destination email address, which is not necessarily unique within a bulk sender.

Also, the SMTP spec was originally written so that teh return path was always the sending/relaying server, so you cannot set this on an incoming message to overwrite it. What actually
happens is incoming Return-Path headers should ALWAYS be ignored and the final delivery agent sets the value of this header to the "envelope from" address, which is how the
relaying server identifies itself when it sends "FROM mailserver.example.com".

The solution is a mail filter, which can be plugged into the *smtpd* pipeline (note: smtp is the outgoing daemon, smtpd is the incoming one!)

This filter, based on the [rust milter crate](https://docs.rs/milter/latest/milter/), uses the incoming header "Set-Return-Path" to overwrite what the server would usually pass as
the "envelope from". The "Set-Return-Path" header is then removed.

## How to use
Currently, you will need to install the rust compiler chain for Linux as well as the build-essential, pkg-config and libmilter-dev packages in order to build it. You then need to run it
(`cargo run` works but I need to work out the best way to run it at startup).

Then you need to edit `/etc/postfix/main.cf` or use `postconf` to add the executable name from the milter (default: inet:3000@localhost) to the configs `smtpd_milters` and `non_smtpd_milters`

You do not normally need to restart postfix for the daemon to spot the config change and reload it.

I do not currently have the answer on how to monitor the milter for problems and restart it. There is so little code in it that it should not fall over, I guess that is more likely
if the calling library has a quirk in some condition.

PRs welcome but I am a noob in Rust so I might not know the answers.
