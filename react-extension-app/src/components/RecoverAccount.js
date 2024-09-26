import React from "react";
import { BulbOutlined } from "@ant-design/icons";
import { Button, Input } from "antd";
import { useNavigate } from "react-router-dom";
import { useState } from "react";
import axios from "axios";

const { TextArea } = Input;

function RecoverAccount({ setWallet, setSeedPhrase }) {
  const navigate = useNavigate();
  const [typedSeed, setTypedSeed] = useState("");
  const [nonValid, setNonValid] = useState(false);
  const [error, setError] = useState("");
  const [passphrase, setPassphrase] = useState('');


  function seedAdjust(e) {
    setNonValid(false);
    setTypedSeed(e.target.value);
  }

  async function recoverWallet(){
    try {
      setError('');
      const response = await axios.post('http://localhost:8080/create_wallet', {
        phrase: typedSeed,
        passphrase: passphrase || null,
      });
      // setPublicKey(response.data.public_key);
      setSeedPhrase(typedSeed);
      setWallet(response.data.public_key)
      navigate("/yourwallet");
    } catch (err) {
      setError('Failed to create wallet. Please check your mnemonic phrase and try again.');
      console.error('Error creating wallet:', err);
    }

    // setSeedPhrase(typedSeed);
    // setWallet(recoveredWallet.address);
    // navigate("/yourwallet");
    return;
  }

  return (
    <>
      <div className="content">
        <div className="mnemonic">
          <BulbOutlined style={{ fontSize: "20px" }} />
          <div>
            Type your seed phrase in the field below to recover your wallet (it
            should include 12 words seperated with spaces)
          </div>
        </div>
        <TextArea
          value={typedSeed}
          onChange={seedAdjust}
          rows={4}
          className="seedPhraseContainer"
          placeholder="Type your seed phrase here..."
        />
        <Button
        disabled={
          typedSeed.split(" ").length !== 12 || typedSeed.slice(-1) === " "
        }
          className="frontPageButton"
          type="primary"
          onClick={() => recoverWallet()}
        >
          Recover Wallet
        </Button>
        {nonValid && <p style={{color: "red"}}> Invalid Seed Phrase</p>}
        <p className="frontPageBottom" onClick={() => navigate("/")}>
          <span>Back Home</span>
        </p>
      </div>
    </>
  );
}

export default RecoverAccount;
