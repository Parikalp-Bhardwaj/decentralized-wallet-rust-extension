import React, { useEffect, useState } from "react";
import {
  Divider,
  Tooltip,
  List,
  Avatar,
  Spin,
  Tabs,
  Input,
  Button,
} from "antd";
import { LogoutOutlined } from "@ant-design/icons";
import { useNavigate } from "react-router-dom";
import logo from "../noImg.png";
import axios from "axios";
import { CHAINS_CONFIG } from "../chains";
import { ethers } from "ethers";

function WalletView({
  wallet,
  setWallet,
  seedPhrase,
  setSeedPhrase,
  selectedChain,
}) {
  const navigate = useNavigate();
  const [tokens, setTokens] = useState(null);
  const [nfts, setNfts] = useState(null);
  const [balance, setBalance] = useState(0);
  const [fetching, setFetching] = useState(true);
  const [amountToSend, setAmountToSend] = useState(null);
  const [sendToAddress, setSendToAddress] = useState(null);
  const [processing, setProcessing] = useState(false);
  const [hash, setHash] = useState(null);

  
  const items = [
    {
      key: "3",
      label: `Tokens`,
      children: (
        <>
          {tokens ? (
            <>
              <List
                bordered
                className="tokenList"
                itemLayout="horizontal"
                dataSource={tokens}
                renderItem={(item, index) => (
                  <List.Item
                    className="tokenName"
                    style={{ textAlign: "left" }}
                  >
                    <div style={{color:"#f7f9f9"}}>
                     {item.mint.slice(0,4)+"..."+item.mint.slice(40,44)}
                    </div>
                    <div className="tokenAmount" style={{ color: "#007BFF" }}>
                      {(
                        Number(item.amount) 
                      ).toFixed(3)}{" "}
                      Tokens
                    </div>
                  </List.Item>
                )}
              />
            </>
          ) : (
            <>
              <span>You seem to not have any tokens yet</span>
            </>
          )}
        </>
      ),
    },
    {
      key: "2",
      label: `NFTs`,
      children: (
        <>
          {nfts ? (
            <>
              {nfts.map((e, i) => {
                return (
                  <>
                    {e && (
                      <img
                        key={i}
                        className="nftImage"
                        alt="nftImage"
                        src={e}
                      />
                    )}
                  </>
                );
              })}
            </>
          ) : (
            <>
              <span>You seem to not have any NFTs yet</span>
            </>
          )}
        </>
      ),
    },
    {
      key: "1",
      label: `Transfer`,
      children: (
        <>
          <h3>Native Balance </h3>
          <h1>
            {balance.toFixed(2)/1_000_000_000} SOL{CHAINS_CONFIG[selectedChain]}
          </h1>
          <div className="sendRow">
            <p style={{ width: "90px", textAlign: "left" }}> To:</p>
            <Input
              value={sendToAddress}
              onChange={(e) => setSendToAddress(e.target.value)}
              placeholder="0x..."
            />
          </div>
          <div className="sendRow">
            <p style={{ width: "90px", textAlign: "left" }}> Amount:</p>
            <Input
              value={amountToSend}
              onChange={(e) => setAmountToSend(e.target.value)}
              placeholder="Native tokens you wish to send..."
            />
          </div>
          <Button
            style={{ width: "100%", marginTop: "20px", marginBottom: "20px" }}
            type="primary"
            onClick={() => sendTransaction(sendToAddress, amountToSend)}
          >
            Send Tokens
          </Button>
          {processing && (
            <>
              <Spin />
              {hash && (
                <Tooltip title={hash}>
                  <p>Hover For Tx Hash</p>
                </Tooltip>
              )}
            </>
          )}
        </>
      ),
    },
  ];

  async function sendTransaction(to, amount) {

    try {
     
      setProcessing(true);
      const privateKeyResponse = await axios.post('http://localhost:8080/get_privateKey', {
        mnemonic: seedPhrase, 
      });

      const privateKey = privateKeyResponse.data.private_key; 
      try {
        if (!privateKey) {
          // setError('Private key is not available. Fetch it first.');
          return;
        }
        console.log("privateKey ", privateKey, "amount ", amount, "to ", to); 
  
        const response = await axios.post('http://127.0.0.1:8080/send_sol', {
          sender_private_key: privateKey, 
          recipient_public_key: to, 
          amount: parseFloat(amount), 
        });
  
        setProcessing(false);
        console.log("respone ",response.data.transaction_signature)
        getAccountTokens();
      } catch (err) {
        console.error(err);
      }

    } catch (err) {
      console.error(err);
    }
  }

  async function getAccountTokens() {
    setFetching(true);

    try {
      const payload = {
        public_key: wallet.trim(),
      };
  
      if (selectedChain.trim()) {
        payload.rpc = selectedChain.trim();
      }
  
      const response = await axios.post('http://localhost:8080/get_balance', payload);
  
      setBalance(response.data.balance);
    } catch (err) {
      // setError('Failed to fetch balance. Please check the public key and try again.');
      console.error('Error fetching balance:', err);
    } 

    setFetching(false);
  }

  function logout() {
    setSeedPhrase(null);
    setWallet(null);
    setNfts(null);
    setTokens(null);
    setBalance(0);
    navigate("/");
  }



  useEffect(() => {
    if (!wallet || !selectedChain) return;
    setNfts(null);
    setTokens(null);
    setBalance(0);
    getAccountTokens();
  }, []);

  useEffect(() => {
    if (!wallet) return;
    setNfts(null);
    setTokens(null);
    setBalance(0);
    getAccountTokens();
  }, [selectedChain]);

  useEffect(() => {
    async function getTokensForSolanaAccount(publicKey,selectedChain) {
      const requestPayload = {
        jsonrpc: "2.0",
        id: 1,
        method: "getTokenAccountsByOwner",
        params: [
          publicKey,
          { "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" }, // Solana Token Program ID
          { "encoding": "jsonParsed" }
        ]
      };
      
      try {
        const response = await axios.post(selectedChain, requestPayload);  // Ensure `selectedChain` is defined elsewhere
        const tokenAccounts = response.data.result.value;

        // Log the token account information
        tokenAccounts.forEach(account => {
          const tokenAmount = account.account.data.parsed.info.tokenAmount;
          console.log(`Mint: ${account.account.data.parsed.info.mint}, Balance: ${tokenAmount.uiAmountString}`);
        });
        const allTokens = tokenAccounts.map(account => ({
          mint: account.account.data.parsed.info.mint,
          amount: account.account.data.parsed.info.tokenAmount.uiAmountString,
        }));


        setTokens(allTokens);
      } catch (error) {
        console.error('Error fetching token accounts:', error);
      }
    }


    getTokensForSolanaAccount(wallet, selectedChain);
  }, [wallet, selectedChain]); 


  


  return (
    <>
      <div className="content">
        <div className="logoutButton" onClick={logout}>
          <LogoutOutlined />
        </div>
        <div className="walletName">Wallet</div>
        <Tooltip title={wallet}>
          <div className="walletAddress">
            {wallet.slice(0, 4)}...{wallet.slice(38)}
          </div>
        </Tooltip>
        <Divider />
        {fetching ? (
          <Spin />
        ) : (
          <Tabs defaultActiveKey="1" items={items} className="walletView" />
        )}
      </div>
    </>
  );
}

export default WalletView;
