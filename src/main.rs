use tokio::time::{sleep, Duration};
use tokio::task;
use colored::*;
use std::fs::File;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::prelude::*;
use std::ffi::CString;
use std::os::raw::c_int;
use libc;
use structopt::StructOpt;
use std::io;
use std::env;
use dialoguer::Select;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Serialize, Deserialize)]
struct Data {
    name: String,
    uid: u32,
    version: u32,
    plan: String
}


#[derive(Debug, StructOpt)]
#[structopt(name = "flint", about = "An example Actix Web application.")]
enum Command {
    #[structopt(name = "init", about = "Initialize the application.")]
    Init,
    #[structopt(name = "rollout", about = "Rollout new branch")]
    Rollout,
    #[structopt(name = "rollback", about = "Rollback the commit ")]
    Rollback,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let command = Command::from_args();

    match command {
        Command::Init => init().await,
        Command::Rollout => rollout().await,
        Command::Rollback => rollback().await,
    }
}

fn generate_random_string(length: usize) -> String {
    let mut rng = thread_rng();
    let random_string: String = rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}

async fn rollout() -> io::Result<()> {
    let random_code = generate_random_string(10);
    let mut success = false;

    // Create a new branch
    let git_branch_command = format!("git branch {}", random_code);
    let git_branch_cstring = CString::new(git_branch_command.clone()).expect("Failed to create CString");

    unsafe {
        let git_branch_command_ptr = git_branch_cstring.into_raw();
        let result = libc::system(git_branch_command_ptr);
        CString::from_raw(git_branch_command_ptr);

        if result != -1 {
            println!("New branch created successfully: {}", random_code);
        } else {
            println!("Failed to create new branch");
            return Ok(());
        }
    }

    // Add all changes
    let git_add_command = "git add .";
    let git_add_cstring = CString::new(git_add_command).expect("Failed to create CString");

    unsafe {
        let git_add_command_ptr = git_add_cstring.into_raw();
        let result = libc::system(git_add_command_ptr);
        CString::from_raw(git_add_command_ptr);

        if result != -1 {
            println!("Changes added successfully");
        } else {
            println!("Failed to add changes");
            return Ok(());
        }
    }

    // Commit changes
    let commit_message = "flint automation";
    let git_commit_command = format!("git commit -m \"{}\"", commit_message);
    let git_commit_cstring = CString::new(git_commit_command).expect("Failed to create CString");

    unsafe {
        let git_commit_command_ptr = git_commit_cstring.into_raw();
        let result = libc::system(git_commit_command_ptr);
        CString::from_raw(git_commit_command_ptr);

        if result != -1 {
            println!("Changes committed successfully");
            success = true;
        } else {
            println!("Failed to commit changes");
        }
    }

    if success {
    let commit_message = "flint automation";
    let git_commit_command = format!("git push origin main");
    let git_commit_cstring = CString::new(git_commit_command).expect("Failed to create CString");

    unsafe {
        let git_commit_command_ptr = git_commit_cstring.into_raw();
        let result = libc::system(git_commit_command_ptr);
        CString::from_raw(git_commit_command_ptr);

        if result != -1 {
            println!("Changes committed successfully");
            success = true;
        } else {
            println!("Failed to commit changes");
        }
    }
    }

    Ok(())
}



async fn vercel() -> std::io::Result<()> {
    let result = task::spawn_blocking(|| {
        if let Ok(current_dir) = env::current_dir() {
            if let Some(dir_name) = current_dir.file_name() {
                if let Some(dir_name_str) = dir_name.to_str() {
                    println!("{} starting..... {}", "[info]".blue(), dir_name_str);
                } else {
                    println!("Directory name is not valid UTF-8");
                }
            } else {
                println!("Failed to retrieve directory name");
            }
        } else {
            println!("Failed to retrieve current directory");
        }
    }).await;

    sleep(Duration::from_secs(5)).await;
    println!("{} ignited", "[info]".blue());
    
   
        

    let data = Data {
        name: "Auth".to_string(),
        uid: 3074841949,
        version: 1,
        plan: "Gold".to_string(),
    };
    
    // Serialize the Person instance to JSON
    let json_data = serde_json::to_string(&data)?;

    // Open or create a file named "data.json" in write mode
    let mut file = File::create("flint.json")?;

    // Write the JSON data to the file
    file.write_all(json_data.as_bytes())?;
    // Call the inner async function
   vercel_start().await?;

    Ok(())
}

async fn vercel_start() -> std::io::Result<()> {
    let mut success = false;

    // Install Vercel globally
    let install_command = CString::new("sudo npm i -g vercel").expect("Failed to create CString");

    unsafe {
        let result = libc::system(install_command.as_ptr());
        if result != -1 {
            success = true;
        } else {
            println!("Failed to execute command");
        }
    }

    // If installation was successful, proceed to login and then to dev
    if success {
        // Log in to Vercel
        let login_command = CString::new("vercel login").expect("Failed to create CString");

        unsafe {
            let result = libc::system(login_command.as_ptr());
            if result == -1 {
                println!("Failed to execute command");
            }
        }

        // Check if login was successful before proceeding to "vercel dev"
        if success {
            // Run "vercel dev"
            let dev_command = CString::new("vercel dev").expect("Failed to create CString");

            unsafe {
                let result = libc::system(dev_command.as_ptr());
                if result == -1 {
                    println!("Failed to execute command");
                }
            }
        }
    }

    Ok(())
}


async fn rollback() -> io::Result<()> {
    let mut success = false;

    // Reset changes to the previous commit
    let git_reset_command = "git reset --hard HEAD^";
    let git_reset_cstring = CString::new(git_reset_command).expect("Failed to create CString");

    unsafe {
        let git_reset_command_ptr = git_reset_cstring.into_raw();
        let result = libc::system(git_reset_command_ptr);
        CString::from_raw(git_reset_command_ptr);

        if result != -1 {
            println!("Changes reset to previous commit");
        } else {
            println!("Failed to reset changes");
            return Ok(());
        }
    }

    // Force push to origin
    let git_force_push_command = "git push origin --force";
    let git_force_push_cstring = CString::new(git_force_push_command).expect("Failed to create CString");

    unsafe {
        let git_force_push_command_ptr = git_force_push_cstring.into_raw();
        let result = libc::system(git_force_push_command_ptr);
        CString::from_raw(git_force_push_command_ptr);

        if result != -1 {
            println!("Changes force pushed to origin");
            success = true;
        } else {
            println!("Failed to force push changes to origin");
        }
    }

    if success {
        // Further steps if the rollback was successful
    }

    Ok(())
}




async fn init() -> io::Result<()> {
    let items = vec!["vercel", "render[soon]", "heroku[soon]"];

    let selection = Select::new()
        .with_prompt("What do you choose?")
        .items(&items)
        .interact()
        .unwrap();

    if items[selection] == "vercel" {
        // Call the asynchronous function and await its execution
        vercel().await?;
    } else {
        println!("idk");
        // Add default routes or handle the case when selection is not "vercel"
    }

    Ok(())
}
