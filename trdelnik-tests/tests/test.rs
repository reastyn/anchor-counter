use fehler::throws;
// @todo: do not forget to import your program crate (also in the ../Cargo.toml)
use anchor_counter::instruction::Increment;
use anchor_counter::{accounts::Update, Counter};
use program_client::anchor_counter_instruction::{self, PROGRAM_ID};
use trdelnik_client::anchor_lang::AccountDeserialize;
use trdelnik_client::solana_sdk::account::ReadableAccount;
use trdelnik_client::{anyhow::Result, *};

// @todo: create and deploy your fixture
#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut validator = Validator::default();

    validator.add_program("anchor_counter", PROGRAM_ID);
    let client = validator.start().await;

    let mut fixture = Fixture::new(client);
    fixture.deploy().await?;
    fixture
}

#[trdelnik_test]
async fn test_increment(#[future] init_fixture: Result<Fixture>) {
    // @todo: add your happy path test scenario and the other test cases
    let fixture = init_fixture.await?;

    anchor_counter_instruction::increment(
        &fixture.client,
        Increment {},
        Update {
            counter: fixture.increment_account.pubkey(),
            user: fixture.client.payer().pubkey(),
            system_program: System::id(),
        },
        vec![
            fixture.client.payer().clone(),
            fixture.increment_account.clone(),
        ],
    )
    .await?;

    let counter_account = fixture
        .client
        .get_account(fixture.increment_account.pubkey())
        .await?
        .unwrap();

    let counter_account = Counter::try_deserialize(
        &mut counter_account.data()
    ).unwrap();
    assert!(counter_account.count == 1);
}

struct Fixture {
    client: Client,
    increment_account: Keypair,
}
impl Fixture {
    fn new(client: Client) -> Self {
        Fixture {
            client,
            increment_account: keypair(1),
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
    }
}
