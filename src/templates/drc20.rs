use super::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PType {
  #[serde(rename = "prc-20")]
  Prc20,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
  Transfer,
  Mint,
  Deploy,
  Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct PRC20 {
  pub p: Option<PType>,
  pub op: Option<Operation>,
  pub tick: Option<String>,
  pub amt: Option<String>,
  pub max: Option<String>,
  pub limit: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct PRC20Balance {
  tick: String,
  transferable: String,
  available: String,
  utxos: Option<Vec<PRC20Output>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct PRC20Output {
  #[serde(flatten)]
  pub utxo: Utxo,
  pub prc20: PRC20UtxoOutput,
  pub inscription_id: InscriptionId,
  pub inscription_number: u64,
  pub offset: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct PRC20UtxoOutput {
  pub balance: String,
  pub operation: Operation,
  pub valid: bool,
}

impl PRC20Balance {
  pub fn from_strings(
    tick: &str,
    transferable: &str,
    available: &str,
    utxos: Vec<PRC20Output>,
  ) -> Option<Self> {
    Some(Self {
      tick: tick.to_string(),
      transferable: transferable.to_string(),
      available: available.to_string(),
      utxos: if utxos.is_empty() { None } else { Some(utxos) },
    })
  }
}

impl PRC20 {
  pub fn from_json_string(json_str: &str) -> Option<Self> {
    match serde_json::from_str::<PRC20>(json_str) {
      Ok(prc20) => {
        if prc20.is_valid() {
          Some(prc20)
        } else {
          None
        }
      }
      Err(err) => {
        log::debug!("Error deserializing JSON: {}", err);
        None
      }
    }
  }

  fn is_valid(&self) -> bool {
    self.p.is_some()
        && self.tick.is_some()
        && self.clone().op.is_some_and(|op| op != Operation::Unknown)
  }
}
