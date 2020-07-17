use super::op::*;

pub struct Config {
    pub packet_type: PacketType,
    pub standby_config: StandbyConfig,
    pub sync_word: u16,
    #[cfg(feature = "tcxo")]
    pub tcxo_delay: TcxoDelay,
    #[cfg(feature = "tcxo")]
    pub tcxo_voltage: TcxoVoltage,
    pub packet_params: PacketParams,
    pub mod_params: ModParams,
    pub tx_params: TxParams,
    pub rf_freq: u32,
}