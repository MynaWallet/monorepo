import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: { 
    version: "0.8.19",
    settings: { 
      optimizer: { 
        enabled: true, 
        runs: 0
      },
    },
  },
  networks: {
    hardhat: {
    // コントラクトサイズが大きい場合は以下をtrueにする
      allowUnlimitedContractSize: true
    },
    // locallhost: {
    //   allowUnlimitedContractSize: true
    // }
  }
};

export default config;
