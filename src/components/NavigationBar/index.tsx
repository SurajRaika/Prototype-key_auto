import React from "react";
import { Link } from "react-router-dom";
import { Button, Text } from "@geist-ui/core";
import { PlusSquare, ChevronsLeft , Pause , RefreshCw,MinusSquare } from "@geist-ui/icons";
import "./style.css";
import PausePlayButton from "../PausePlayButton";
const Navbar = () => {
    //   const history = useHistory();

    const handleBackClick = () => {
        // history.goBack();
    };

    const handleCreateClick = () => {
        // Handle create button click here
        console.log("Create button clicked");
    };

    return (
        <nav id="NavigationTopBar">
            <span>SchedulePro</span>
            <span>

 
            <Button style={{ borderRadius: 2 }} icon={<MinusSquare />} auto scale={2 / 4} placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined} >Hide </Button>
<PausePlayButton/>

            </span>

        </nav>
    );
};

export default Navbar;
