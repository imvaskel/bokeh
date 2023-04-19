"use client";

import { ChangeEvent, DragEvent, FormEvent, useRef, useState } from "react";
import styles from "./page.module.css";

export default function Home() {
  const [file, setFile] = useState("");
  const [mediaHref, setMediaHref] = useState("");
  const [errorText, setErrorText] = useState("");
  const fileInputRef = useRef<HTMLInputElement>(null);

  const overrideDefaults = (ev: DragEvent<HTMLDivElement>) => {
    ev.preventDefault();
    ev.stopPropagation();
  };

  const forwardRefClick = () => {
    fileInputRef.current?.click();
  };

  const handleOnChange = (event: ChangeEvent<HTMLInputElement>) => {
    if (event.target.files && event.target.files[0]) {
      setFile(event.target.files[0].name);
    }
  }

  const forwardDropHandler = (event: DragEvent<HTMLDivElement>) => {
    event.stopPropagation();
    event.preventDefault();

    if (event.dataTransfer?.files) {
      let input = fileInputRef.current;
      if (input !== null) {
        input.files = event.dataTransfer.files;
      }
      let file = event.dataTransfer.files[0];
      if (file) {
        setFile(file.name);
      }
    }
  };

  const onSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    event.stopPropagation();

    setErrorText("");
    setMediaHref("");
    const data = new FormData(event.target as HTMLFormElement);
    const key = data.get("key");
    data.delete("key");

    if (!key) {
      setErrorText(`You must enter a key.`);
      return;
    }

    try {
      const res = await fetch("/media/upload", {
        method: "POST",
        headers: new Headers({
          Authorization: "Bearer " + key,
        }),
        body: data,
      });
      const json = await res.json();
      if (res.status === 401) {
        // Oops! Unauthorized.
        setErrorText(`An error occurred when submitting: ${json.msg}`);
      }
      else if (res.status !== 200) {
        setErrorText(`An error occurred when submitting ${json.msg}`)
      }
      else {
        setMediaHref(`/media/${json.msg}`);
      }
    } catch (e) {
      setErrorText(`An error occurred when submitting: ${e}`);
    }
  };

  return (
    <main>
      <div className={styles.upload_container}>
        <div className={styles.card}>
          <div className={styles.header_container}>
            <h1>Upload</h1>
          </div>
          <form onSubmit={onSubmit}>
            <div
              className={styles.file_upload_wrapper}
              onClick={forwardRefClick}
              onDrop={forwardDropHandler}
              onDragEnter={overrideDefaults}
              onDragLeave={overrideDefaults}
              onDragOver={overrideDefaults}
            >
              <label htmlFor="file">
                {file !== "" ? file : "Drag a File or Click Here"}
              </label>
              <input
                onChange={handleOnChange}
                type="file"
                name="file"
                className={styles.file_upload}
                ref={fileInputRef}
              />
            </div>
            <div className={styles.form_footer_wrapper}>
              <input
                type="text"
                name="key"
                id="key"
                className={styles.key_entry}
                placeholder="API Key"
              />
              <button className={styles.submit_button}>Submit</button>
            </div>
          </form>
        </div>

        {mediaHref && (
          <div className={`${styles.card} ${styles.success_card}`}>
            <h2>Success!</h2>
            <p className={styles.card_text}>Your image URL is ready.</p>
            <a href={mediaHref} className={styles.unstyled_anchor}>
              {window.location.protocol}//{window.location.hostname}{mediaHref}
            </a>
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
