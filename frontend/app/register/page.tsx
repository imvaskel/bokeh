"use client";

import { FormEvent, useState } from "react";
import styles from "./page.module.css";

export default function Register() {
  const [key, setKey] = useState("");
  const [errorText, setErrorText] = useState("");

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    setKey(""); // reset both popups to not display anything.
    setErrorText("");

    event.preventDefault();
    const target = event.currentTarget;

    const username = target.username.value;
    const inviteKey = target.key.value;

    if (!username) {
      setErrorText("You must provide a username!")
      return;
    }
    else if (!inviteKey) {
      setErrorText("You must provide an invite key!")
      return;
    }

    try {
      const res = await fetch("/user/register", {
        method: "POST",
        headers: new Headers({
          "Content-Type": "application/json",
        }),
        body: JSON.stringify({
          username: username,
          key: inviteKey,
        }),
      });
      if (res.ok) {
        const json = await res.json();
        setKey(json.msg);
      }
      else if (res.status === 401) {
        setErrorText("The provided key was invalid.");
      }
    } catch (error) {
      setErrorText("There was an error when registering your account.")
    }
  };

  return (
    <main>
      <div className={styles.register_container}>
        <div className={styles.card}>
          <h1>Register</h1>
          <form onSubmit={handleSubmit}>
            <input
              type="text"
              placeholder="Username"
              id="username"
              className={styles.text_input}
            />
            <input
              type="text"
              placeholder="Invite Key"
              id="key"
              className={styles.text_input}
            />
            <button type="submit" className={styles.submit_button}>
              Submit
            </button>
          </form>
        </div>
        {key && (
          <div className={`${styles.card} ${styles.success_card}`}>
            <h1>Success!</h1>
            <p>Your account is registered.</p>
            <p className={styles.success_text}>{key}</p>
          </div>
        )}

        {errorText && (
          <div className={`${styles.card} ${styles.error_card}`}>
            <h2>Error!</h2>
            <p className={styles.card_text}>{errorText}</p>
          </div>
        )}
      </div>
    </main>
  );
}
