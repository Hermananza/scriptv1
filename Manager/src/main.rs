mod funcs;
mod text_funcs;

use std::{env, fs, io, thread};
use std::io::BufRead;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use chrono::DateTime;
use rusqlite::Connection;
use crate::text_funcs::{text_to_bold};
use crate::funcs::{create_user, change_limit, change_pass, change_validity, expired_report_json, expired_report_vec, generate_test, is_port_avaliable, remove_user, user_already_exists, users_report_json, users_report_vec, run_command_and_get_output, get_connections, enable_badvpn_port, disable_badvpn_port, enable_proxy_port, disable_proxy_port, online_report_json, online_report, userdata, speedtest_data, enable_checkuser_port, disable_checkuser_port, journald_status, disable_journald, enable_journald, get_services, enable_openvpn, disable_openvpn, restore_backup, make_backup, enable_sslproxy_port, disable_sslproxy_port};

fn main() {
    let sqlite_conn = Connection::open("/opt/rustymanager/db").unwrap();
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        main_menu(&sqlite_conn);
    } else{
        match (&args[1]).as_str() {
            "--create-user" => {
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("user empty");
                        return;
                    }
                    _i if 3 >= _i  => {
                        println!("pass empty");
                        return;
                    }
                    _i if 4 >= _i => {
                        println!("days empty");
                        return;
                    }
                    _i if 5 >= _i => {
                        println!("limit empty");
                        return;
                    }
                    _ => {}
                }


                let days = &args[4];
                let limit = &args[5];

                match days.parse::<usize>() {
                    Ok(..) => {}
                    Err(..) => {
                        println!("invalid digit found in days");
                        return
                    }
                }
                match limit.parse::<usize>() {
                    Ok(..) => {}
                    Err(..) => {
                        println!("invalid digit found in limit");
                        return
                    }
                }

                let string = create_user(&args[2], &args[3], days.parse().unwrap(), limit.parse().unwrap(), false, &sqlite_conn);
                println!("{}", string)

            }
            "--remove-user" => {
                if 2 >= args.len() {
                    println!("user empty");
                    return;
                }
                let string = remove_user(&args[2], false, &sqlite_conn);
                println!("{}", string);
            }
            "--generate-test" => {
                if 2 >= args.len() {
                    println!("minutes empty");
                    return;
                }

                let days = &args[2];

                match days.parse::<usize>() {
                    Ok(..) => {}
                    Err(..) => {
                        println!("invalid digit found in days");
                        return
                    }
                }

                let string = generate_test(days.parse().unwrap(), &sqlite_conn);
                println!("{}", string);
            }
            "--change-limit" => {
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("user empty");
                        return;
                    }
                    _i if 3 >= _i  => {
                        println!("limit empty");
                        return;
                    }
                    _ => {}
                }

                let limit = &args[3];

                match limit.parse::<usize>() {
                    Ok(..) => {}
                    Err(..) => {
                        println!("invalid digit found in limit");
                        return
                    }
                }

                let string = change_limit(&args[2], limit.parse().unwrap(), false, &sqlite_conn);
                println!("{}", string);
            }
            "--change-validity" => {
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("user empty");
                        return;
                    }
                    _i if 3 >= _i  => {
                        println!("days empty");
                        return;
                    }
                    _ => {}
                }

                let days = &args[3];

                match days.parse::<usize>() {
                    Ok(..) => {}
                    Err(..) => {
                        println!("invalid digit found in days");
                        return
                    }
                }

                let string = change_validity(&args[2], days.parse().unwrap(), false, &sqlite_conn);
                println!("{}", string);
            }
            "--change-pass" => {
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("user empty");
                        return;
                    }
                    _i if 3 >= _i  => {
                        println!("pass empty");
                        return;
                    }
                    _ => {}
                }


                let string = change_pass(&args[2], &args[3], false, &sqlite_conn);
                println!("{}", string);
            }
            "--userdata" => {
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("user empty");
                        return;
                    }
                    _ => {}
                }

                let string = userdata(&args[2], &sqlite_conn);
                println!("{}", string);
            }

            "--users-report" => {
                let string = users_report_json(&sqlite_conn);
                println!("{}", string);
            }

            "--expired-report" => {
                let string = expired_report_json(&sqlite_conn);
                println!("{}", string);
            }

            "--online-report" => {
                let string = online_report_json(&sqlite_conn);
                println!("{}", string);
            }
            "--remove-expired" => {
                let expired = expired_report_vec(&sqlite_conn);
                if expired.len() > 0 {
                    for user in expired {
                        remove_user(user.user.as_str(), true, &sqlite_conn);
                        println!("removed")
                    }
                } else {
                    println!("not found")
                }
            }
            "--make-backup" => {
                let make = make_backup(&sqlite_conn);
                println!("{}", make);
            }
            "--restore-backup" => {
                match args.len() {
                    _i if 2 >= _i  => {
                        println!("path empty");
                        return;
                    }
                    _ => {}
                }
                let backup_path = &args[2];
                if Path::new(&backup_path).exists() {
                    let restore = restore_backup(&sqlite_conn, backup_path.to_string());
                    if restore == "backup restored" {
                        println!("{}", restore);
                    }
                } else {
                    println!("file not found in path");
                }

            }
            "--help" => {
                let mut text = " -- help data".to_owned();
                text = text + "\n   --create-user <user> <pass> <days> <limit>";
                text = text + "\n   --remove-user <user>";
                text = text + "\n   --generate-test <time in minutes>";
                text = text + "\n   --change-limit <user> <limit>";
                text = text + "\n   --change-validity <user> <validity in days>";
                text = text + "\n   --change-pass <user> <pass>";
                text = text + "\n   --users-report";
                text = text + "\n   --expired-report";
                text = text + "\n   --online-report";
                text = text + "\n   --make-backup";
                text = text + "\n   --restore-backup <backup path>";

                println!("{}", text)
            }

            _ => {
                println!("função invalida selecionada")
            }
        }
    }
}

fn user_dont_exists() {
    println!("esse não existe\n\n> Tekan tombol apa pun untuk kembali ke menu");
    let mut return_string = String::new();
    io::stdin().read_line(&mut return_string).expect("");
}

fn user_exists() {
    println!("esse usuario já existe\n\n> Tekan tombol apa pun untuk kembali ke menu");
    let mut return_string = String::new();
    io::stdin().read_line(&mut return_string).expect("");
}

fn get_os_info() -> (String, String) {
    let os_info = fs::read_to_string("/etc/os-release").expect("Failed to read /etc/os-release");

    let mut os_name = String::new();
    let mut os_version = String::new();

    for line in os_info.lines() {
        if line.starts_with("ID=") {
            os_name = line.trim_start_matches("ID=").trim_matches('"').to_string();
        } else if line.starts_with("VERSION_ID=") {
            os_version = line.trim_start_matches("VERSION_ID=").trim_matches('"').to_string();
        }
    }

    (os_name, os_version)
}


fn main_menu(sqlite_conn: &Connection) {
    loop {
        
        Command::new("clear").status().unwrap();
        println!("{}", text_to_bold("Calculando uso de cpu e ram..."));
        let (os, version) = get_os_info();
        let ssh_online = run_command_and_get_output("ps -e -o user= -o cmd= | grep '[s]shd: ' | grep -v 'sshd: root@' | awk '{user=$1; if (user != \"root\" && user != \"sshd\") print user}' | wc -l");
        let ovpn_online = run_command_and_get_output("sed -n '/Common Name/,/ROUTING TABLE/{/Common Name/d;/ROUTING TABLE/q;s/,.*//p}' /etc/openvpn/openvpn-status.log 2>/dev/null | wc -l || echo 0");
        let online = ssh_online.parse::<usize>().unwrap() + ovpn_online.parse::<usize>().unwrap();

        let created = run_command_and_get_output("awk -F: '$3 >= 1000 { C++ } END { print C+0 }' /etc/passwd");
        let cpu_usage = run_command_and_get_output("vmstat 1 2 | tail -n 1 | awk '{print 100 - $15 \"%\"}'");
        let cpu_cores = run_command_and_get_output("nproc");
        let ram_total = run_command_and_get_output("free -m | awk 'NR==2{print $2 \" MB\"}'");
        let ram_usage = run_command_and_get_output("free -m | awk 'NR==2{printf \"%.2f%%\\n\", $3*100/$2}'");

        Command::new("clear").status().unwrap();
        println!("{}", text_to_bold("================= RustyManager ================="));
        println!("------------------------------------------------");
        println!("| {} {:<16} | {} {:<3} |", text_to_bold("Os:"), os, text_to_bold("Jumlah Pengguna:"), created);
        println!("| {} {:<12} | {} {:<4} |", text_to_bold("Versi: "), version, text_to_bold("Pengguna Online:"), online);
        println!("-----------------------|------------------------");
        println!("| {:<28} | {:<29} |", text_to_bold("CPU:"), text_to_bold("Ram:"));
        println!("|  - {} {:<8} |  - {} {:<11} |", text_to_bold("Cpu  :"), cpu_cores, text_to_bold("Total:"), ram_total);
        println!("|  - {} {:<9} |  - {} {:<10} |", text_to_bold("Digunakan:"), cpu_usage, text_to_bold("Digunakan:"), ram_usage);
        println!("------------------------------------------------");
        let options = vec![
            "Kelola Pengguna",
            "Kelola Koneksi",
            "Peralatan",
        ];

        for (i, option) in options.iter().enumerate() {
            println!("| {:02} - {:<39} |", i + 1, option);
        }
        println!("| 00 - {:<39} |", "Sair");
        println!("------------------------------------------------");
        println!("\n --> Pilih Opsi:");

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    0 => { break }
                    1 => {
                        users_menu(&sqlite_conn);
                    }
                    2 => {
                        connection_menu(&sqlite_conn);
                    }
                    3 =>{
                        utils_menu(&sqlite_conn)
                    }

                    _ => {}
                }
            }
            Err(err) => {
                println!("{}", err);
                println!("Pilih Opsi valida")
            }
        }
    }
}
fn users_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|              {}              |", text_to_bold("Kelola Pengguna"));
        println!("------------------------------------------------");
        let options = vec![
            "Buat Akun",
            "Hapus Akun",
            "Gerar teste",
            "Ubah limit",
            "Ubah validasi",
            "Ubah Sandi",
            "Laporan Pengguna",
            "Laporan Exp",
            "Laporan Online",
            "Hapus Exp"
        ];

        for (i, option) in options.iter().enumerate() {
            println!("| {:02} - {:<39} |", i + 1, option);
        }
        println!("| 00 - {:<39} |", "Sair");
        println!("------------------------------------------------");
        println!("\n --> Pilih Opsi:");

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();


        match option.trim().parse() {
            Ok(op) => {
                match op {
                    0 => { break }
                    1 => {
                        Command::new("clear").status().unwrap();
                        println!("--> função selecionada: criar usuario");
                        println!("Digite o usuario: ");
                        let mut user = String::new();
                        io::stdin().read_line(&mut user).unwrap();
                        user = user.trim().to_string();
                        if user.is_empty() {
                            continue;
                        }

                        if user_already_exists(user.as_str()) {
                            user_exists();
                            continue
                        }

                        println!("Digite a senha: ");
                        let mut pass = String::new();
                        io::stdin().read_line(&mut pass).unwrap();
                        pass = pass.trim().to_string();
                        if pass.is_empty() {
                            continue;
                        }

                        let mut days = String::new();
                        loop {
                            println!("Digite a expiração em dias: ");
                            if !days.is_empty() {
                                days = String::new()
                            }
                            io::stdin().read_line(&mut days).unwrap();
                            days = days.trim().to_string();
                            match days.parse::<usize>() {
                                Ok(..) => {
                                    break
                                }
                                Err(..) => {
                                    println!("digite um numero valido");
                                }
                            }
                        }

                        let mut limit = String::new();
                        loop {
                            println!("Digite o limite de conexões: ");
                            if !limit.is_empty() {
                                limit = String::new()
                            }
                            io::stdin().read_line(&mut limit).unwrap();
                            limit = limit.trim().to_string();
                            match limit.parse::<usize>() {
                                Ok(..) => {
                                    break
                                }
                                Err(..) => {
                                    println!("digite um numero valido");
                                }
                            }
                        }
                        Command::new("clear").status().unwrap();

                        let create = create_user(&*user, &*pass, days.parse().unwrap(), limit.parse().unwrap(), true, &sqlite_conn);
                        match create.as_str() {
                            "created" => {
                                let mut text = ">>> Usuario criado com Sukses".to_owned();
                                text = text + "\n - Pengguna: " + &*user;
                                text = text + "\n - Sandi: " + &*pass;
                                text = text + "\n - Exp: " + &*days;
                                text = text + "\n - Limit: " + &*limit;
                                text = text + "\n\n> Tekan tombol apa pun untuk kembali ke menu";
                                println!("{}", text);
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }

                            "error on insert user in db" => {
                                println!("o usuario foi criado, mas ocorreu um erro para salvar ele na db\n\n> Tekan tombol apa pun untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }
                            _ => {}
                        }
                    }
                    2 => {
                        Command::new("clear").status().unwrap();
                        println!("--> função selecionada: remover usuario");
                        println!("Digite o usuario: ");
                        let mut user = String::new();
                        io::stdin().read_line(&mut user).unwrap();
                        user = user.trim().to_string();
                        if user.is_empty() {
                            continue;
                        }

                        if !user_already_exists(user.as_str()) {
                            user_dont_exists();
                            continue
                        }

                        let remove = remove_user(&*user, true, &sqlite_conn);
                        match remove.as_str() {
                            "removed" => {
                                println!(">>> Usuario removido com Sukses\n\n> Tekan tombol apa pun untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }
                            "error on remove user at db" => {
                                println!("o usuario foi removido, mas ocorreu um erro ao tentar remover ele na db\n\n> Tekan tombol apa pun untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }
                            _ => {}
                        }
                    }
                    3 => {
                        Command::new("clear").status().unwrap();
                        println!("--> função selecionada: gerar teste");
                        let mut minutes = String::new();
                        loop {
                            println!("Digite o tempo de expiração em minutos: ");
                            if !minutes.is_empty() {
                                minutes = String::new()
                            }
                            io::stdin().read_line(&mut minutes).unwrap();
                            minutes = minutes.trim().to_string();
                            match minutes.parse::<usize>() {
                                Ok(..) => {
                                    break
                                }
                                Err(..) => {
                                    println!("digite um numero valido");
                                }
                            }
                        }

                        let gen = generate_test(minutes.parse().unwrap(), &sqlite_conn);
                        match gen.as_str() {
                            "error on insert user in db" => {
                                Command::new("clear").status().unwrap();
                                println!("o teste foi criado, mas ocorreu um erro para salvar ele na db\n\n> Tekan tombol apa pun untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }
                            _ => {
                                if gen.contains("user: ") {
                                    Command::new("clear").status().unwrap();
                                    let mut text = ">>> Teste gerado com Sukses".to_owned();
                                    let user = gen.split("user: ").collect::<Vec<&str>>()[1].split(" |").collect::<Vec<&str>>()[0];
                                    text = text + "\n - Pengguna: " + &*user;
                                    text = text + "\n - Sandi: " + &*user;
                                    text = text + "\n - Exp: " + &*minutes + " minutos";
                                    text = text + "\n - Limite de conexões: 1";
                                    text = text + "\n\n> Tekan tombol apa pun untuk kembali ke menu";
                                    println!("{}", text);
                                    let mut return_string = String::new();
                                    io::stdin().read_line(&mut return_string).expect("");
                                }

                            }
                        }
                    }
                    4 => {
                        Command::new("clear").status().unwrap();
                        println!("--> função selecionada: alterar limite");
                        println!("Digite o usuario: ");
                        let mut user = String::new();
                        io::stdin().read_line(&mut user).unwrap();
                        user = user.trim().to_string();
                        if user.is_empty() {
                            continue;
                        }

                        if !user_already_exists(user.as_str()) {
                            user_dont_exists();
                            continue
                        }

                        let mut limit = String::new();
                        loop {
                            println!("Digite o novo limit: ");
                            if !limit.is_empty() {
                                limit = String::new()
                            }
                            io::stdin().read_line(&mut limit).unwrap();
                            limit = limit.trim().to_string();
                            match limit.parse::<usize>() {
                                Ok(..) => {
                                    break
                                }
                                Err(..) => {
                                    println!("digite um numero valido");
                                }
                            }
                        }

                        let change = change_limit(&*user, limit.parse().unwrap(), false, &sqlite_conn);
                        match change.as_str() {
                            "error on update user in db" => {
                                Command::new("clear").status().unwrap();
                                println!("ocorreu algum erro, tente novamente\n\n> Tekan tombol apa pun untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }

                            _ => {
                                if change.contains("changed") {
                                    Command::new("clear").status().unwrap();
                                    println!(">>> Limite alterado com Sukses\n\n> Tekan tombol apa pun untuk kembali ke menu");
                                    let mut return_string = String::new();
                                    io::stdin().read_line(&mut return_string).expect("");
                                }

                            }
                        }


                    }
                    5 => {
                        Command::new("clear").status().unwrap();
                        println!("--> função selecionada: alterar validade");
                        println!("Digite o usuario: ");
                        let mut user = String::new();
                        io::stdin().read_line(&mut user).unwrap();
                        user = user.trim().to_string();
                        if user.is_empty() {
                            continue;
                        }

                        if !user_already_exists(user.as_str()) {
                            user_dont_exists();
                            continue
                        }

                        let mut days = String::new();
                        loop {
                            println!("Digite a nova validade em dias: ");
                            if !days.is_empty() {
                                days = String::new()
                            }
                            io::stdin().read_line(&mut days).unwrap();
                            days = days.trim().to_string();
                            match days.parse::<usize>() {
                                Ok(..) => {
                                    break
                                }
                                Err(..) => {
                                    println!("digite um numero valido");
                                }
                            }
                        }

                        let change = change_validity(&*user, days.parse().unwrap(), false, &sqlite_conn);
                        match change.as_str() {
                            "error on update user in db" => {
                                Command::new("clear").status().unwrap();
                                println!("a validade foi alterada, mas ocorreu algum erro ao tentar atualizar ele na db\n\n> Tekan tombol apa pun untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }

                            _ => {
                                if change.contains("changed") {
                                    Command::new("clear").status().unwrap();
                                    println!(">>> Validade alterada com Sukses\n\n> Tekan tombol apa pun untuk kembali ke menu");
                                    let mut return_string = String::new();
                                    io::stdin().read_line(&mut return_string).expect("");
                                }
                            }
                        }
                    }
                    6 => {
                        Command::new("clear").status().unwrap();
                        println!("--> função selecionada: alterar senha");
                        println!("Digite o usuario: ");
                        let mut user = String::new();
                        io::stdin().read_line(&mut user).unwrap();
                        user = user.trim().to_string();
                        if user.is_empty() {
                            continue;
                        }

                        if !user_already_exists(user.as_str()) {
                            user_dont_exists();
                            continue
                        }

                        let mut pass = String::new();
                        println!("Digite a nova senha: ");
                        io::stdin().read_line(&mut pass).unwrap();
                        pass = pass.trim().to_string();


                        let change = change_pass(&*user, &*pass, false, &sqlite_conn);
                        match change.as_str() {
                            "error on update user in db" => {
                                Command::new("clear").status().unwrap();
                                println!("a senha foi alterada, mas ocorreu algum erro ao tentar atualizar ele na db\n\n> Tekan tombol apa pun untuk kembali ke menu");
                                let mut return_string = String::new();
                                io::stdin().read_line(&mut return_string).expect("");
                            }

                            _ => {
                                if change.contains("changed") {
                                    Command::new("clear").status().unwrap();
                                    println!(">>> Senha alterada com Sukses\n\n> Tekan tombol apa pun untuk kembali ke menu");
                                    let mut return_string = String::new();
                                    io::stdin().read_line(&mut return_string).expect("");
                                }
                            }
                        }
                    }
                    7 => {
                        Command::new("clear").status().unwrap();
                        println!("--> função selecionada: relatorio de usuarios");
                        let users = users_report_vec(&sqlite_conn);
                        for user in users {
                            println!("Usuario: {} | Senha: {} | Limite: {} | Expira em: {}", user.user, user.pass, user.limit, DateTime::parse_from_str(&user.expiry, "%Y-%m-%d %H:%M:%S%.3f %z").unwrap().format("%Y-%m-%d"));
                        }
                        println!("\n> Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    8 => {
                        Command::new("clear").status().unwrap();
                        println!("--> função selecionada: relatorio de usuarios expirados");
                        let expired = expired_report_vec(&sqlite_conn);
                        for user in expired {
                            println!("Usuario: {} | Senha: {} | Limite: {} | Expira em: {}", user.user, user.pass, user.limit, DateTime::parse_from_str(&user.expiry, "%Y-%m-%d %H:%M:%S%.3f %z").unwrap().format("%Y-%m-%d"));
                        }
                        println!("\n> Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    9 => {
                        let stdin = io::stdin();
                        let handle = thread::spawn(move || {
                            let _ = stdin.lock().lines().next();
                        });

                        loop {
                            Command::new("clear").status().unwrap();
                            println!("Memantau pengguna melalui SSH");
                            println!("------------------------------------------");
                            println!("Pengguna           | Koneksi/Limit");
                            println!("--------------------------");

                            let users = online_report(&sqlite_conn);
                            let mut total_connections: usize = 0;
                            for user in users {
                                total_connections += user.connected.parse::<usize>().unwrap();
                                println!("{:<18} | {}/{}", user.user, user.connected, user.limit);
                            }
                            if total_connections != 0 {
                                println!("--------------------------");
                            }
                            println!("Total de conexões: {}", total_connections);
                            println!("\n> Tekan tombol apa pun untuk kembali ke menu");

                            if handle.is_finished() {
                                break;
                            }
                            thread::sleep(Duration::from_secs(1));
                        }
                    }
                    10 => {
                        Command::new("clear").status().unwrap();
                        println!("--> função selecionada: remover usuarios expirados");
                        let expired = expired_report_vec(&sqlite_conn);
                        if expired.len() > 0 {
                            for user in expired {
                                remove_user(user.user.as_str(), true, &sqlite_conn);
                                println!("usuario: {} removido", user.user);
                            }
                        } else {
                            println!("nenhum usuario expirado encontrado")
                        }
                        println!("\n> Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    _ => {}
                }
            }
            Err(err) => {
                println!("{}", err);
                println!("Pilih Opsi valida")
            }
        }
    }
}

fn connection_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|              {}              |", text_to_bold("Gerenciar Conexões"));
        println!("------------------------------------------------");
        println!("| 1 - {:<40} |", "Port Aktif");
        println!("| 2 - {:<40} |", "RustyProxy (ws/wss/socks)");
        println!("| 3 - {:<40} |", "RustyProxySSL (direct/ws/wss)");
        println!("| 4 - {:<40} |", "Badvpn");
        println!("| 5 - {:<40} |", "OpenVpn");
        println!("| 0 - {:<40} |", "Kembali Ke Menu");
        println!("------------------------------------------------");
        let mut option = String::new();
        println!("\n --> Pilih Opsi:");
        io::stdin().read_line(&mut option).unwrap();

        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        services_menu()
                    }
                    2 => {
                        proxy_menu(&sqlite_conn)
                    }
                    3 => {
                        sslproxy_menu(&sqlite_conn)
                    }
                    4 => {
                        badvpn_menu(&sqlite_conn)
                    }
                    5 => {
                        openvpn_menu(&sqlite_conn)
                    }
                    0 => {
                        break
                    }
                    _ => {
                        Command::new("clear").status().unwrap();
                        println!("\n> Opção invalida, Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                }
            }
            Err(_) => {
                Command::new("clear").status().unwrap();
                println!("\n> Opção invalida, Tekan tombol apa pun untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }


    }
}

fn utils_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|                  {}                 |", text_to_bold("Ferramentas"));
        println!("------------------------------------------------");
        println!("| {:<45}|", "1 - Chek user Multi-Apps");
        println!("| {:<45}|", "2 - Test Kecepatan");
        println!("| {:<45}|", "3 - Pantau Sumberdaya");
        println!("| {:<45}|", "4 - Kelola Jurnal");
        println!("| {:<45}|", "5 - Buat Cadangkan");
        println!("| {:<45}|", "6 - Kembalikan Cadangkan");
        println!("| {:<45}|", "7 - Ubah kata sandi root");
        println!("| {:<45}|", "8 - Restart server");
        println!("| {:<45}|", "0 - Kembali Ke Menu");
        println!("------------------------------------------------");
        println!();
        let mut option = String::new();
        println!(" --> Pilih Opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        checkuser_menu(&sqlite_conn);
                    }
                    2 => {
                        Command::new("clear").status().unwrap();
                        println!("teste em execução, essa ação pode demorar...");
                        let speedtest = speedtest_data();
                        let download_bits = speedtest.download.bytes as f64 * 8.0;
                        let upload_bits = speedtest.upload.bytes as f64 * 8.0;

                        let download_mb = download_bits / 1_000_000.0;
                        let upload_mb = upload_bits / 1_000_000.0;

                        let download_seconds = speedtest.download.elapsed as f64 / 1000.0;
                        let upload_seconds = speedtest.upload.elapsed as f64 / 1000.0;

                        let download_mbps = download_mb / download_seconds;
                        let upload_mbps = upload_mb / upload_seconds;

                        Command::new("clear").status().unwrap();

                        println!("------------------------------------------------");
                        println!("|              {}             |", text_to_bold("Teste de Velocidade"));
                        println!("------------------------------------------------");
                        println!("| Rede: {:<38} |", speedtest.interface.name);
                        println!("| Ip: {:<40} |", speedtest.interface.internal_ip);
                        println!("| Download: {:<34} |", format!("{:.2}mbps", download_mbps));
                        println!("| Upload:   {:<34} |", format!("{:.2}mbps", upload_mbps));
                        println!("| Ping:     {:<32}   |", format!("{:.2}ms", speedtest.ping.latency));
                        println!("------------------------------------------------");

                        println!("\n> Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    3 => {
                        Command::new("clear").status().unwrap();
                        println!("{}", text_to_bold("> aviso: para sair do monitor, pressione F10"));
                        println!("> pressione qualquer tecla para continuar");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                        Command::new("htop").status().unwrap();
                    }
                    4 => {
                        journald_menu();
                    }
                    5 => {
                        Command::new("clear").status().unwrap();
                        println!("{}", text_to_bold("gerando backup..."));
                        let make = make_backup(&sqlite_conn);
                        if make == "backup done in /root/backup.json" {
                            println!("backup criado com Sukses, salvo em: /root/backup.json")
                        }
                        println!("> Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    6 => {
                        Command::new("clear").status().unwrap();
                        println!("Digite o caminho do arquivo:");
                        let mut backup_path = String::new();
                        io::stdin().read_line(&mut backup_path).expect("");
                        backup_path = backup_path.trim().to_string();

                        if Path::new(&backup_path).exists() {
                            println!("arquivo encontrado, restaurando backup...\n");
                            let restore = restore_backup(&sqlite_conn, backup_path.to_string());
                            if restore == "backup restored" {
                                println!("\nbackup restaurado com Sukses");
                            }
                        } else {
                            println!("\no arquivo não foi encontrado no caminho digitado");
                        }
                        println!("> Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    7 => {
                        Command::new("clear").status().unwrap();
                        loop {
                            println!("Digite a nova senha root:");
                            let mut new_pass = String::new();
                            io::stdin().read_line(&mut new_pass).expect("");
                            new_pass = new_pass.trim().to_string();
                            if new_pass.len() >= 4 {
                                run_command_and_get_output(format!("(echo {}; echo {}) | passwd root", new_pass, new_pass).as_str());
                                println!("senha alterada");
                                break;
                            }
                        }
                        println!("> Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    8 => {
                        Command::new("reboot").status().unwrap();
                    }

                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opção invalida, Tekan tombol apa pun untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}
fn proxy_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();

        println!("------------------------------------------------");
        println!("|                  {}                 |", text_to_bold("RUSTY PROXY"));
        println!("------------------------------------------------");
        let conn = get_connections(&sqlite_conn).unwrap();
        let proxy_ports = conn.proxy.ports.unwrap_or_default();
        if proxy_ports.is_empty() {
            println!("| Portas(s): {:<34}|", "tidak ada");
        } else {
            let active_ports = proxy_ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ");
            println!("| Portas(s): {:<34}|", active_ports);
        }

        println!("------------------------------------------------");
        println!("| {:<45}|", "1 - Membuka Port");
        println!("| {:<45}|", "2 - Menutup Port");
        println!("| {:<45}|", "0 - Kembali Ke Menu");
        println!("------------------------------------------------");
        println!();
        let mut option = String::new();
        println!(" --> Pilih Opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukan Port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if !is_port_avaliable(port).unwrap() {
                                        println!("essa porta já está em uso, digite outra:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("Masukan Port Yang Benar");
                                }
                            }

                        }
                        println!("Digite o status de conexão (não digite nada para o padrão): ");
                        let mut status = String::new();
                        io::stdin().read_line(&mut status).unwrap();
                        status = status.trim().to_string();

                        enable_proxy_port(port, status);
                        Command::new("clear").status().unwrap();
                        println!("\n> Porta ativada com Sukses, Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    2 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukan Port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if is_port_avaliable(port).unwrap() {
                                        println!("essa porta não está em uso, digite outra:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("Masukan Port Yang Benar");
                                }
                            }

                        }

                        disable_proxy_port(port);
                        Command::new("clear").status().unwrap();
                        println!("\n> Porta desativada com Sukses, Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opção invalida, Tekan tombol apa pun untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}

fn sslproxy_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();

        println!("------------------------------------------------");
        println!("|                  {}              |", text_to_bold("RUSTY PROXY SSL"));
        println!("------------------------------------------------");
        let conn = get_connections(&sqlite_conn).unwrap();
        let sslproxy_ports = conn.sslproxy.ports.unwrap_or_default();
        if sslproxy_ports.is_empty() {
            println!("| Portas(s): {:<34}|", "tidak ada");
        } else {
            let active_ports = sslproxy_ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ");
            println!("| Portas(s): {:<34}|", active_ports);
        }

        println!("------------------------------------------------");
        println!("| {:<45}|", "1 - Membuka Port");
        println!("| {:<45}|", "2 - Menutup Port");
        println!("| {:<45}|", "0 - Kembali Ke Menu");
        println!("------------------------------------------------");
        println!();
        let mut option = String::new();
        println!(" --> Pilih Opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukan Port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if !is_port_avaliable(port).unwrap() {
                                        println!("essa porta já está em uso, digite outra:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("Masukan Port Yang Benar");
                                }
                            }

                        }
                        enable_sslproxy_port(port);
                        Command::new("clear").status().unwrap();
                        println!("\n> Porta ativada com Sukses, Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    2 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukan Port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if is_port_avaliable(port).unwrap() {
                                        println!("essa porta não está em uso, digite outra:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("Masukan Port Yang Benar");
                                }
                            }

                        }

                        disable_sslproxy_port(port);
                        Command::new("clear").status().unwrap();
                        println!("\n> Porta desativada com Sukses, Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opção invalida, Tekan tombol apa pun untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}
fn badvpn_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|                    {}                    |", text_to_bold("BADVPN"));
        println!("------------------------------------------------");
        let conn = get_connections(&sqlite_conn).unwrap();
        let badvpn_ports = conn.badvpn.ports.unwrap_or_default();
        if badvpn_ports.is_empty() {
            println!("| Portas(s): {:<34}|", "tidak ada");
        } else {
            let active_ports = badvpn_ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ");
            println!("| Portas(s): {:<34}|", active_ports);
        }

        println!("| 1 - {:<40} |", "Membuka Port");
        println!("| 2 - {:<40} |", "Menutup Port");
        println!("| 0 - {:<40} |", "Kembali Ke Menu");
        println!("------------------------------------------------");
        let mut option = String::new();
        println!("\n --> Pilih Opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukan Port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if !is_port_avaliable(port).unwrap() {
                                        println!("essa porta já está em uso, digite outra:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("Masukan Port Yang Benar");
                                }
                            }

                        }

                        enable_badvpn_port(port);

                        Command::new("clear").status().unwrap();
                        println!("\n> Porta ativada com Sukses, Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    2 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukan Port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if is_port_avaliable(port).unwrap() {
                                        println!("essa porta não está em uso, digite outra:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("Masukan Port Yang Benar");
                                }
                            }

                        }

                        disable_badvpn_port(port);

                        Command::new("clear").status().unwrap();
                        println!("\n> Porta desativada com Sukses, Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opção invalida, Tekan tombol apa pun untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}
fn checkuser_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|                   {}                  |", text_to_bold("CHECKUSER"));
        println!("------------------------------------------------");
        let conn = get_connections(&sqlite_conn).unwrap();
        let checkuser_ports = conn.checkuser.ports.unwrap_or_default();
        if checkuser_ports.is_empty() {
            println!("| Portas(s): {:<34}|", "tidak ada");
        } else {
            let active_ports = checkuser_ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ");
            println!("| Portas(s): {:<34}|", active_ports);
        }
        println!("| 1 - {:<40} |", "Membuka Port");
        println!("| 2 - {:<40} |", "Menutup Port");
        println!("| 0 - {:<40} |", "Kembali Ke Menu");
        println!("------------------------------------------------");
        let mut option = String::new();
        println!("\n --> Pilih Opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukan Port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if !is_port_avaliable(port).unwrap() {
                                        println!("essa porta já está em uso, digite outra:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("Masukan Port Yang Benar");
                                }
                            }

                        }

                        enable_checkuser_port(port);

                        Command::new("clear").status().unwrap();
                        println!("\n> Porta ativada com Sukses, Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    2 => {
                        let mut port = String::new();
                        loop {
                            println!("Masukan Port: ");
                            if !port.is_empty() {
                                port = String::new();
                            };
                            io::stdin().read_line(&mut port).unwrap();
                            port = port.trim().to_string();
                            match port.parse::<usize>() {
                                Ok(port) => {
                                    if is_port_avaliable(port).unwrap() {
                                        println!("essa porta não está em uso, digite outra:")
                                    } else {
                                        break
                                    }
                                }
                                Err(..) => {
                                    println!("Masukan Port Yang Benar");
                                }
                            }

                        }

                        disable_checkuser_port(port);

                        Command::new("clear").status().unwrap();
                        println!("\n> Porta desativada com Sukses, Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");

                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opção invalida, Tekan tombol apa pun untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}

fn openvpn_menu(sqlite_conn: &Connection) {
    loop {
        Command::new("clear").status().unwrap();

        println!("------------------------------------------------");
        println!("|                    {}                   |", text_to_bold("OPENVPN"));
        println!("------------------------------------------------");
        let conn = get_connections(&sqlite_conn).unwrap();
        let openvpn_port = conn.openvpn.port.unwrap_or_default();
        if openvpn_port.is_empty() {
            println!("| Porta: {:<38}|", "tidak ada");
            println!("| 1 - {:<40} |", "Aktifkan OpenVPN");
        } else {
            println!("| Porta: {:<38}|", openvpn_port);
            println!("| 1 - {:<40} |", "DesAktifkan OpenVPN");
        }
        println!("| 0 - {:<40} |", "Kembali Ke Menu");
        println!("------------------------------------------------");
        let mut option = String::new();
        println!("\n --> Pilih Opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        if openvpn_port.is_empty() {
                            let mut port = String::new();
                            loop {
                                println!("Masukan Port: ");
                                if !port.is_empty() {
                                    port = String::new();
                                };
                                io::stdin().read_line(&mut port).unwrap();
                                port = port.trim().to_string();
                                match port.parse::<usize>() {
                                    Ok(port) => {
                                        if !is_port_avaliable(port).unwrap() {
                                            println!("essa porta já está em uso, digite outra:")
                                        } else {
                                            break
                                        }
                                    }
                                    Err(..) => {
                                        println!("Masukan Port Yang Benar");
                                    }
                                }

                            }

                            let mut mode = String::new();
                            loop {
                                println!("Digite o modo (tcp ou udp): ");
                                if !mode.is_empty() {
                                    mode = String::new();
                                };
                                io::stdin().read_line(&mut mode).unwrap();
                                mode = mode.trim().to_string();
                                println!("modo selecionado: {}", mode);
                                if mode == "tcp" || mode == "udp" {
                                    break
                                }
                            }

                            enable_openvpn(port, mode);

                            Command::new("clear").status().unwrap();
                            println!("\n> OpenVPN Diaktifkan com Sukses");
                            println!("\n> Certificado salvo em: /root/client.ovpn, Tekan tombol apa pun untuk kembali ke menu");
                            let mut return_string = String::new();
                            io::stdin().read_line(&mut return_string).expect("");

                        } else {
                            disable_openvpn();
                            Command::new("clear").status().unwrap();
                            println!("\n> OpenVPN Dinonaktifkan com Sukses, Tekan tombol apa pun untuk kembali ke menu");
                            let mut return_string = String::new();
                            io::stdin().read_line(&mut return_string).expect("");
                        }

                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opção invalida, Tekan tombol apa pun untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}

fn journald_menu() {
    loop {
        Command::new("clear").status().unwrap();
        
        println!("------------------------------------------------");
        println!("|               {}              |", text_to_bold("Gerenciar Journald"));
        println!("------------------------------------------------");
        let status = journald_status();
        if status {
            println!("| Status: {:<37}|", "Diaktifkan");
            println!("------------------------------------------------");
            println!("| 1 - {:<40} |", "DesAktifkan");
        } else {
            println!("| Status: {:<37}|", "Dinonaktifkan");
            println!("------------------------------------------------");
            println!("| 1 - {:<40} |", "Aktifkan");
        }
        println!("| 0 - {:<40} |", "Kembali Ke Menu");
        println!("------------------------------------------------");
        let mut option = String::new();
        println!("\n --> Pilih Opsi:");
        io::stdin().read_line(&mut option).unwrap();
        match option.trim().parse() {
            Ok(op) => {
                match op {
                    1 => {
                        if status {
                            disable_journald()
                        } else {
                            enable_journald()
                        }
                        Command::new("clear").status().unwrap();
                        println!("\n> Sukses, Tekan tombol apa pun untuk kembali ke menu");
                        let mut return_string = String::new();
                        io::stdin().read_line(&mut return_string).expect("");
                    }
                    0 => {
                        break
                    }
                    _ => {
                        continue
                    }
                }
            }
            _ => {
                Command::new("clear").status().unwrap();
                println!("\n> Opção invalida, Tekan tombol apa pun untuk kembali ke menu");
                let mut return_string = String::new();
                io::stdin().read_line(&mut return_string).expect("");
            }
        }
    }
}
fn services_menu() {
    Command::new("clear").status().unwrap();

    println!("------------------------------------------------");
    println!("|                 {}                |", text_to_bold("Portas Ativas"));
    println!("------------------------------------------------");
    let services = get_services();
    for service in services {
        println!("| - {:<43}|", format!("{}: {}", service.name, service.ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" ")));
    }
    println!("------------------------------------------------");
    println!();
    println!("> Tekan tombol apa pun untuk kembali ke menu");
    let mut return_string = String::new();
    io::stdin().read_line(&mut return_string).expect("");
}
