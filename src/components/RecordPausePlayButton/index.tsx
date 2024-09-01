import React from 'react';
import { Button } from '@geist-ui/core';
import { Pause, Play } from '@geist-ui/icons';
import { invoke } from '@tauri-apps/api/tauri';

interface RecordPausePlayButtonProps {
    isRecording: boolean;
    setIsRecording: React.Dispatch<React.SetStateAction<boolean>>;
}

const RecordPausePlayButton: React.FC<RecordPausePlayButtonProps> = ({ isRecording, setIsRecording }) => {
    // Handler to toggle between pause and play
    const toggleRecording = async () => {
        const newState = !isRecording;
        setIsRecording(newState);
        console.log(newState ? 'Start Recording' : 'Stop Recording');

        try {
            if (newState) {
                await invoke('start_record');
            } else {
                await invoke('stop_record');
            }
        } catch (error) {
            console.error('Error invoking Tauri command:', error);
        }
    };

    return (
        <Button
            shadow
            style={{ borderRadius: 2 }}
            icon={isRecording ? <Pause /> : <Play />}
            auto
            scale={2 / 4}
            type={isRecording ? 'error' : 'success'}
            onClick={toggleRecording} placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}        >
            {isRecording ? 'Stop Recording' : 'Record'}
        </Button>
    );
};

export default RecordPausePlayButton;