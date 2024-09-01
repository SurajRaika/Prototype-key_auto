import React, { useState, useEffect, useRef } from 'react';
import { Input } from '@geist-ui/core';
import { invoke } from '@tauri-apps/api/tauri';
import { Event, listen } from '@tauri-apps/api/event';
import { ParallelActions } from "../types";

interface ShortcutKeyInputProps {
    value: ParallelActions | null;
    onChange: (value: ParallelActions | null) => void;
}

const ShortcutKeyInput: React.FC<ShortcutKeyInputProps> = ({ value, onChange }) => {
    const [isListening, setIsListening] = useState(false);
    const unlisten = useRef<(() => void) | null>(null);

    useEffect(() => {
        async function setupListener() {
            if (isListening && !unlisten.current) {
                unlisten.current = await listen<string>('updateCounter', (event: Event<string>) => {
                    const keyPresses: ParallelActions = JSON.parse(event.payload);
                    onChange(keyPresses);
                });
            } else if (!isListening && unlisten.current) {
                unlisten.current();
                unlisten.current = null;
            }
        }

        setupListener();

        return () => {
            if (unlisten.current) {
                unlisten.current();
            }
        };
    }, [isListening, onChange]);

    const handleFocus = async () => {
        setIsListening(true);
        try {
            await invoke('start_record');
        } catch (error) {
            console.error('Error starting record:', error);
        }
    };

    const handleBlur = async () => {
        setIsListening(false);
        try {
            await invoke('stop_record');
        } catch (error) {
            console.error('Error stopping record:', error);
        }
    };

    return (
        <Input
            value={value ? JSON.stringify(value) : ''}
            readOnly
            placeholder="Click here to set shortcut"
            onFocus={handleFocus}
            onBlur={handleBlur}
            width="100%" onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined} crossOrigin={undefined}        />
    );
};

export default ShortcutKeyInput;