import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "hardhat-gas-reporter"
import dotenv from "dotenv";
dotenv.config();

const COINMARKETCAP_API_KEY = process.env.COINMARKETCAP_API_KEY || "";

const config: HardhatUserConfig = {
  solidity: { 
    version: "0.8.19",
    settings: { 
      // optimizer: { 
      //   enabled: true, 
      //   runs: 0
      // },
    },
  },
  networks: {
    hardhat: {
    // コントラクトサイズが大きい場合は以下をtrueにする
      allowUnlimitedContractSize: true
    },
  },
  gasReporter: {
    enabled: process.env.REPORT_GAS ? true : false,
    currency: "JPY",
    gasPriceApi:
      "https://api.etherscan.io/api?module=proxy&action=eth_gasPrice",
    coinmarketcap: COINMARKETCAP_API_KEY,
  },
};

export default config;