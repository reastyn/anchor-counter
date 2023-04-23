use anchor_counter::{accounts::Update, instruction::{Increment, Decrement}, Counter};
use program_client::anchor_counter_instruction::{self, PROGRAM_ID};
use trdelnik_client::{anchor_lang::AccountDeserialize, solana_sdk::account::ReadableAccount, *};
use trdelnik_fuzz::{FuzzTestBuilder, State};

#[derive(Debug)]
struct CopyableKeypair(Keypair);

impl Clone for CopyableKeypair {
    fn clone(&self) -> Self {
        Self(self.0.insecure_clone())
    }
}

#[derive(Clone, Debug)]
struct TestState {
    count: i128,
    counter_account: CopyableKeypair,
}

fn initialize_validator() -> Validator {
    let mut validator = Validator::default();
    validator.add_program("anchor_counter", PROGRAM_ID);
    validator
}

async fn flow_increment(State(mut state): State<TestState>, client: Client) {
    anchor_counter_instruction::increment(
        &client,
        Increment {},
        Update {
            counter: state.counter_account.0.pubkey(),
            user: client.payer().pubkey(),
            system_program: System::id(),
        },
        vec![client.payer().clone(), state.counter_account.0.clone()],
    )
    .await
    .unwrap();
    state.count += 1;
}

async fn flow_decrement(State(mut state): State<TestState>, client: Client) {
    anchor_counter_instruction::decrement(
        &client,
        Decrement {},
        Update {
            counter: state.counter_account.0.pubkey(),
            user: client.payer().pubkey(),
            system_program: System::id(),
        },
        vec![client.payer().clone(), state.counter_account.0.clone()],
    )
    .await
    .unwrap();
    state.count -= 1;
}

async fn invariant_check_counter(State(state): State<TestState>, client: Client) {
    let counter_account = client
        .get_account(state.counter_account.0.pubkey())
        .await
        .unwrap()
        .unwrap();

    let counter_account = Counter::try_deserialize(&mut counter_account.data()).unwrap();
    assert_eq!(counter_account.count as i128, state.count);
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    FuzzTestBuilder::new()
        .initialize_validator(initialize_validator)
        .add_flow(flow_increment)
        .add_flow(flow_decrement)
        .with_state(TestState {
            counter_account: CopyableKeypair(Keypair::new()),
            count: 0,
        })
        .add_invariant(invariant_check_counter)
        .start(2, 250)
        .await;
}
