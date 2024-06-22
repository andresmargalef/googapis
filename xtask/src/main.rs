use std::{env, fs, path::PathBuf};

mod gen;

fn main() {
    match env::args().nth(1) {
        Some(cmd) => match cmd.as_str() {
            "gen" => gen(),
            _ => print_help(),
        },
        _ => print_help(),
    }
}

fn print_help() {
    println!(r#"cargo xtask gen"#)
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    Client,
    Server,
}

impl Mode {
    fn gen_proto_folder(&self) -> &str {
        match self {
            Mode::Client => "googapis/genproto/clients",
            Mode::Server => "googapis/genproto/servers",
        }
    }
    fn output_rs(&self) -> &str {
        match self {
            Mode::Client => "googapis/src/googapis_clients.rs",
            Mode::Server => "googapis/src/googapis_servers.rs",
        }
    }
    pub fn include_proto(&self) -> &str {
        match self {
            Mode::Client => "include_proto_client",
            Mode::Server => "include_proto_server",
        }
    }
}

fn gen() {
    let proto_root = PathBuf::from("xtask/proto/googleapis");

    // let gates = gen::feature_gates(&protos);
    // println!("{}", gates);

    fn generate(mode: Mode, proto_root: PathBuf, protos: &Vec<gen::Proto>) {
        let out_dir = PathBuf::from(mode.gen_proto_folder());
        let _ = fs::remove_dir_all(out_dir.as_path());
        let _ = fs::create_dir(out_dir.as_path());
        let includes = [proto_root];

        let mut config = prost_build::Config::new();
        config.protoc_arg("--experimental_allow_proto3_optional");

        tonic_build::configure()
            .build_client(Mode::Client == mode)
            .build_server(Mode::Server == mode)
            .out_dir(out_dir.clone())
            .compile_with_config(config, &gen::proto_path(protos), &includes)
            .unwrap();
        println!("{}", out_dir.to_str().unwrap());

        let mut out_path = PathBuf::from(mode.output_rs());
        let root = gen::from_protos(protos);
        fs::write(out_path.clone(), root.gen_code(mode)).unwrap();

        out_path.pop();
        println!("{}", out_path.to_str().unwrap());
    }

    let protos = gen::find_proto(proto_root.clone());
    generate(Mode::Client, proto_root.clone(), &protos);
    generate(Mode::Server, proto_root.clone(), &protos);
}
