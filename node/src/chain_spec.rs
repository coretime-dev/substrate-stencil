use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};
use node_template_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, SessionConfig, StakingConfig, DemocracyConfig, 
	ElectionsConfig, CouncilConfig, TechnicalCommitteeConfig, opaque::SessionKeys,
	StakerStatus, Balance, currency::DOLLARS, WASM_BINARY, Signature, ImOnlineConfig,
};
use sp_consensus_babe::{AuthorityId as BabeId};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};
use sc_service::ChainType;
use hex_literal::hex;
use telemetry::TelemetryEndpoints;
use pallet_staking::Forcing;

// Note this is the URL for the telemetry server
const TAO_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Babe
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		|| testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None,
	)
}

pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		|| testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None,
	)
}

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online }
}

fn testnet_genesis(initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool) -> GenesisConfig {
	
	const STASH: Balance = 100 * DOLLARS;
	let num_endowed_accounts = endowed_accounts.len();
	
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		babe: Some(BabeConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
				))
			}).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		democracy: Some(DemocracyConfig::default()),
		elections_phragmen: Some(ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, STASH))
						.collect(),
		}),
		collective_Instance1: Some(CouncilConfig::default()),
		collective_Instance2: Some(TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		}),
		membership_Instance1: Some(Default::default()),
		treasury: Some(Default::default()),
	}
}

// public staging network
pub fn tao_staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Tao Staging Testnet",
		"tao_staging",
		ChainType::Live,
		tao_staging_testnet_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(TAO_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Westend Staging telemetry url is valid; qed")
		),
		Some("tao_staging"),
		None,
		Default::default(),
	)
}

fn tao_staging_testnet_genesis() -> GenesisConfig {
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5FemZuvaJ7wVy4S49X7Y9mj7FyTR4caQD5mZo2rL7MXQoXMi
		hex!["9eaf896d76b55e04616ff1e1dce7fc5e4a417967c17264728b3fd8fee3b12f3c"].into(),
		// 5FNrxGpnd3z5NTBEFDarNeCCYYx2Fw7DFbsXv1VuwmNXQsNW
		hex!["928dbd595055b13e3606618516e69d60ea4d8861f0f1c632cf9f503c45f24717"].into(),
		// 5DknRrEh2khKAiEV9rFFGJLiQQahSJZ7hTYPQNfYmxFsLHQr
		hex!["4acd70cdbe4a0ab21e96615e1d3f7f809d44ceb169d19232327dc71819451c6e"].into(),
		// 5E4gXxnM9oC16VCfiL2rGbYhie9B1W8unKpQ2HPzj2EqoGJL
		hex!["587403d0dbdc7d12ce2a4da526b18df0a3b5c7c2074464c4879ef47b42769b2d"].into(),
	];

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
	)> = vec![(
		// 5Grpw9i5vNyF6pbbvw7vA8pC5Vo8GMUbG8zraLMmAn32kTNH
		hex!["d41e0bf1d76de368bdb91896b0d02d758950969ea795b1e7154343ee210de649"].into(),
		// 5DLMZF33f61KvPDbJU5c2dPNQZ3jJyptsacpvsDhwNS1wUuU
		hex!["382bd29103cf3af5f7c032bbedccfb3144fe672ca2c606147974bc2984ca2b14"].into(),
		// 5Dhd2QbrSE4dyNn3YUg8j5TY3fG7ZAWZMoRRF9KUc7VPVGmC
		hex!["48640c12bc1b351cf4b051ac1cf7b5740765d02e34989d0a9dd935ce054ebb21"].unchecked_into(),
		// 5C6rkxAZB437B5Bf1yS4B4qjW4HZPeBp8Kzx2Se9FLKhfyHY
		hex!["01a474a93a0cf830fb40b1d17fd1fc7c6b4a95fa11f90345558574a72da0d4b1"].unchecked_into(),
		// 5DscuovXyY1o7DxYroYjYgipn87eqYLyQA3HJ21Utb7TqAai
		hex!["50041e469c63c994374a2829b0b0829213abd53be5113e751043318a9d7c0757"].unchecked_into(),
	),(
		// 5CFDk3yCSgQ2goiaksMfRMFRS7ZU28BZqPQDeAsgZUa6FRzt
		hex!["08050f1b6bcd4651004df427c884073652bafd54e5ca25cea69169532db2910b"].into(),
		// 5F1ks2enazaPktQa3HURLK8GywzNZaGirovPtFvvbv91TLhJ
		hex!["8275157f2a1d8373106cb00078a73a92a3303f3bf6eb72c3a67413bd943b020b"].into(),
		// 5CQ7gVQj96m8y79qPCqrM291rSNREfZ1Tf2fiLPSJReWTNy2
		hex!["0ecddcf7643a98de200b80fe7b18ebd38987fa106c5ed84fc004fa75ea4bac67"].unchecked_into(),
		// 5FyNaMc6GaioN7K9QzPJDEtGThJ1HmcruRdgtiRxaoAwn2VD
		hex!["acdfcce0e40406fac1a8198c623ec42ea13fc627e0274bbb6c21e0811482ce13"].unchecked_into(),
		// 5EUhcM9WPJGvhCz1UptA7ye8TgktGqbhaeSohCkAfW76q5bS
		hex!["6ac58683d639d3992a0090ab15f8c1dcf5a5ab7652fc9de60845441f9fc93903"].unchecked_into(),
	),(
		// 5F6YideXfGcskpdFUczu3nZcJFmU9WKHgjjNVQjqgeVGRs66
		hex!["861c6d95051f942bb022f13fc2125b2974933d8ab1441bfdee9855e9d8051556"].into(),
		// 5F92x4qKNYaHtfp5Yy7kb9r6gHCHkN3YSvNuedERPHgrURTn
		hex!["8801f479e09a78515f1badee0169864dae45648109091e29b03a7b4ea97ec018"].into(),
		// 5CLqVJSpfAdMYW1FHygEV8iEi8XFornEcCzrhw9WmFbbp8Qp
		hex!["0c4d9de1e313572750abe19140db56433d20e4668e09de4df81a36566a8f2528"].unchecked_into(),
		// 5HEQh8yEv4QU7joBCKYdjJJ57qU1gDAm4Xv5QZKfFnSbXpeo
		hex!["e493d74f9fa7568cca9dd294c9619a54c2e1b6bd3ecf3677fa7f9076b98c3fcd"].unchecked_into(),
		// 5GUEUCusMfW9c229gyuDG6XUH9pi3Cs4EZR9STtw8opfKuS6
		hex!["c2e2a133b23995a48ff46cc704ef61929ee4a29b5fa468e41019ac63f3694e1f"].unchecked_into(),
	),(
		// 5FxxpyvEnE2sVujvhr6x4A4G171uv4WKSLvrUNst9M8MfdpV
		hex!["ac8fdba5bbe008f65d0e85181daa5443c2eb492fea729a5981b2161467f8655c"].into(),
		// 5FxFAYsTNf31D5AGbXW9ETZPUZofpreHjJkdKehidcvDt5X4
		hex!["ac039bef73f76755d3747d711554f7fb0f16022da51483e0d600c9c7c8cbf821"].into(),
		// 5GdjiBeMEFqTE6mWod3UqPrtkQTscRGtAcmdSbR26vGiXpwB
		hex!["ca2245b6fa117fab9353a2031104d1d5d62e311957f375762324e65d71127465"].unchecked_into(),
		// 5DMfkaaR4tzmarUsRMkrbnFNmVnYtYjTPFJsjvA4X15WAZZB
		hex!["392c51bf0c08f89cb1e091782d81359475d780986968ba7f6fa60f41feda6bf7"].unchecked_into(),
		// 5HGzdyJakxDdnERv3nvNjd6Xmz5R39NEuuJ2B3miubDY6BHD
		hex!["e68c9a2ee25e1999a4e87906aea429f3e5f3fc8dc9cd89f423d82860c6937b2e"].unchecked_into(),
	)];
	
	const ENDOWMENT: u128 = 1_000_000 * DOLLARS;
	const STASH: u128 = 100 * DOLLARS;
	let num_endowed_accounts = endowed_accounts.len();
	
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		}),
		babe: Some(BabeConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		sudo: Some(SudoConfig {
			key: endowed_accounts[0].clone(),
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(
					x.0.clone(),
					x.0.clone(),
					session_keys(x.2.clone(), x.3.clone(), x.4.clone())
				)
			}).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		democracy: Some(DemocracyConfig::default()),
		elections_phragmen: Some(ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, STASH))
						.collect(),
		}),
		collective_Instance1: Some(CouncilConfig::default()),
		collective_Instance2: Some(TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		}),
		membership_Instance1: Some(Default::default()),
		treasury: Some(Default::default()),
	}
}

