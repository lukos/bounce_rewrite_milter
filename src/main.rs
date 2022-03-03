use milter::*;

fn main() {
	// Unix sockets are probably faster but there are some conditions that leak the socket so using a socket
	// means a bit more work externally to keep an eye on things
    Milter::new("inet:3000@localhost")
        .name("BounceRewriteFilter")
        .on_header(header_callback)
        .on_eom(eom_callback)
        .on_abort(abort_callback)
        .actions(Actions::REPLACE_SENDER | Actions::REPLACE_HEADER)
        .run()
        .expect("milter execution failed");
}

#[on_header(header_callback)]
fn handle_header<'a>(mut context: Context<String>, header: &str, value: &'a str) -> milter::Result<Status> {
    if header == "Set-Return-Path" {
        match context.data.borrow_mut() {
            Some(retpath) => {
                *retpath = value.to_owned();
            },
            None => {
                context.data.replace(value.to_owned())?;
            }
        }
    }

    Ok(Status::Continue)
}

#[on_eom(eom_callback)]
fn handle_eom(mut context: Context<String>) -> milter::Result<Status> {
    match context.data.take() {
        Ok(result) => {
            match result {
                Some(retval) => {
					// Set the Return-Path to the value we obtained earlier and remove the original header
                    context.api.replace_sender(&*retval, None::<&str>)?;
                    context.api.replace_header("Set-Return-Path", 1, None::<&str>)?;
                },
                None => {}
            }
        }
        Err(_error) => {}
    }

    Ok(Status::Continue)
}

#[on_abort(abort_callback)]
fn handle_abort(mut context: Context<String>) -> Status {
	// We need to call take to remove any of our data to avoid memory leaks
    let _ = context.data.take();

    Status::Continue
}
