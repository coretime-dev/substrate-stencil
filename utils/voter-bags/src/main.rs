// use std::path::PathBuf;
// use clap::{ArgEnum, Parser};
// use generate_bags::{generate_thresholds};
// use node_template_runtime::Runtime as MainnetRuntime;

// // 1 Unit = 1e9
// const UNIT: u128 = 1_000_000;
// const TOTAL_ISSUANCE: u128 = UNIT * 1_000_000_000;
// const EXISTENTIAL_DEPOSIT: u128 = UNIT * 10;
// const N_BAGS: usize = 200;

// fn main() {
// 	generate_thresholds::<Runtime>(
// 		N_BAGS,
// 		Path::new("voter_bags.rs"),
// 		TOTAL_ISSUANCE,
// 		EXISTENTIAL_DEPOSIT,
// 	);
// }

// #[derive(Clone, Debug, ArgEnum)]
// #[clap(rename_all = "PascalCase")]
// enum Runtime {
// 	Mainnet,
// }

// impl Runtime {
// 	fn generate_thresholds_fn(
// 		&self,
// 	) -> Box<dyn FnOnce(usize, &Path, u128, u128) -> Result<(), std::io::Error>> {
// 		match self {
// 			Runtime::Mainnet => Box::new(generate_thresholds::<MainnetRuntime>),
// 		}
// 	}
// }

// #[derive(Debug, Parser)]
// struct Opt {
// 	/// How many bags to generate.
// 	#[clap(long, default_value = "200")]
// 	n_bags: usize,

// 	/// Which runtime to generate.
// 	#[clap(long, ignore_case = true, arg_enum, default_value = "Polkadot")]
// 	runtime: Runtime,

// 	/// Where to write the output.
// 	output: PathBuf,

// 	/// The total issuance of the native currency.
// 	#[clap(short, long)]
// 	total_issuance: u128,

// 	/// The minimum account balance (i.e. existential deposit) for the native currency.
// 	#[clap(short, long)]
// 	minimum_balance: u128,
// }

// fn main() -> Result<(), std::io::Error> {
// 	let Opt { n_bags, output, runtime, total_issuance, minimum_balance } = Opt::parse();

// 	runtime.generate_thresholds_fn()(n_bags, &output, total_issuance, minimum_balance)
// }



fn main() {}
