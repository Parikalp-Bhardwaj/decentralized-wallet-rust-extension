import React, { useState } from "react";
import { Button, Card, Tooltip, message } from "antd";
import { ExclamationCircleOutlined } from "@ant-design/icons";
import { useNavigate } from "react-router-dom";
import { ethers } from "ethers";
import { BiCopy } from "react-icons/bi";
import { CopyToClipboard } from "react-copy-to-clipboard";
import axios from "axios";

function CreateAccount({ setWallet, setSeedPhrase }) {
  const [newSeedPhrase, setNewSeedPhrase] = useState(null);
  const navigate = useNavigate();
  const [error, setError] = useState("");
  const [passphrase, setPassphrase] = useState('');

 
  async function generateWallet() {
    try {
      setError("");
      const response = await fetch(
        "http://127.0.0.1:8080/generate_mnemonic",
        {
          method: "POST",
        }
      );
      const data = await response.json();
      setNewSeedPhrase(data.mnemonic);
    } catch (err) {
      setError("Failed to generate mnemonic. Please try again.");
      console.error("Error generating mnemonic:", err);
    }
  }

  async function setWalletAndMnemonic() {
    setSeedPhrase(newSeedPhrase);
    // setWallet(ethers.Wallet.fromPhrase(newSeedPhrase).address);

    try {
      setError('');
      const response = await axios.post('http://localhost:8080/create_wallet', {
        phrase: newSeedPhrase,
        passphrase: passphrase || null,
      });
      // setPublicKey(response.data.public_key);
      setWallet(response.data.public_key)
    } catch (err) {
      setError('Failed to create wallet. Please check your mnemonic phrase and try again.');
      console.error('Error creating wallet:', err);
    }

  }

  return (
    <>
      <div className="content">
        <div className="mnemonic">
          <ExclamationCircleOutlined style={{ fontSize: "20px" }} />
          <div>
            Once you generate the seed phrase, save it securely in order to
            recover your wallet in the future.
          </div>
        </div>
        <Button
          className="frontPageButton"
          type="primary"
          onClick={generateWallet}
        >
          Generate Seed Phrase
        </Button>
        <Card className="seedPhraseContainer">
          {newSeedPhrase && (
            <div style={{ position: "relative" }}>
              <pre style={{ whiteSpace: "pre-wrap" }}>{newSeedPhrase}</pre>
              <CopyToClipboard text={newSeedPhrase}>
                <Tooltip title="Copy Seed Phrase">
                  <BiCopy
                    style={{
                      cursor: "pointer",
                      position: "absolute",
                      right: "10px",
                      top: "10px",
                      fontSize: "24px",
                    }}
                    onClick={() =>
                      message.success("Seed phrase copied to clipboard!")
                    }
                  />
                </Tooltip>
              </CopyToClipboard>
            </div>
          )}
        </Card>
        <Button
          className="frontPageButton"
          type="default"
          onClick={setWalletAndMnemonic}
        >
          Open Your New Wallet
        </Button>
        <p className="frontPageBottom" onClick={() => navigate("/")}>
          Back Home
        </p>
      </div>
    </>
  );
}

export default CreateAccount;
