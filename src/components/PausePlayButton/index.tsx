import React, { useState } from 'react';
import { Button } from '@geist-ui/core';
import { Pause, Play } from '@geist-ui/icons';
import { invoke } from '@tauri-apps/api/tauri';

const PausePlayButton = () => {
    // State to keep track of whether it's paused or playing
    const [isPlaying, setIsPlaying] = useState(false);

    // Handler to toggle between pause and play
    const togglePlayback = async () => {
        const newState = !isPlaying;
        setIsPlaying(newState);
        console.log(newState ? 'Play' : 'Pause');

        try {
            if (newState) {
                await invoke('Play');
            } else {
                await invoke('Pause');
            }
        } catch (error) {
            console.error('Error invoking Tauri command:', error);
        }
    };

    return (
        <Button
            shadow
            style={{ borderRadius: 2 }}
            icon={isPlaying ? <Pause /> : <Play />}
            auto
            scale={2 / 4}
            type={isPlaying ? 'error' : 'success'}
            onClick={togglePlayback} placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}        >
            {isPlaying ? 'Pause' : 'Play'}
        </Button>
    );
};

export default PausePlayButton;
