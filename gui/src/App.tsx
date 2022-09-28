import React, { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
    const [dotfiles, setDotfiles] = useState();

    useEffect(() => {
        invoke("load_dotfiles").then((res: any) => {
            setDotfiles(res);
        });
    }, []);

    return (
        <div>
            <h1>hi</h1>
        </div>
    );
}

export default App;
