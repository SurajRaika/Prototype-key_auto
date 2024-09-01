import { useState } from 'react';
import { Description, Button } from "@geist-ui/core";
import { PlayFill, PauseFill, Delete } from "@geist-ui/icons";
import { EventKey, EventKeys } from "../../types";
import { invoke } from '@tauri-apps/api/tauri';
import { appLocalDataDir, join } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';

interface ActionCardProps {
    eventKey: EventKey;
    name: string;
    description: string;
    id: number;
}





const ActionCard: React.FC<ActionCardProps> = ({ eventKey, name, description, id }) => {
    const [loading, setLoading] = useState(false);
    const [isActive, setIsActive] = useState(false);
    const [show, setShow] = useState(true);


async function handleDelete(id: number) {
    try {
        const localDataDir = await appLocalDataDir();
        const filePath = await join(localDataDir, 'EventKeys.json');
        const fileContents = await readTextFile(filePath);
        const eventKeys: EventKeys = JSON.parse(fileContents);

        // Filter out the event key with the given id
        const updatedEventKeys = eventKeys.filter(eventKey => eventKey.id !== id);

        await writeTextFile(filePath, JSON.stringify(updatedEventKeys));
        setShow(false);
    } catch (error) {
        console.error('Error Removing EventKey:', error);
        throw error;
    }
}
    
    const handleAction = async () => {
        setLoading(true);
        try {
            if (isActive) {
                await invoke('remove_active_event_key', { name: JSON.stringify(eventKey) });
                setIsActive(false);
            } else {
                await invoke('add_active_event_key', { name: JSON.stringify(eventKey) });
                setIsActive(true);

            }
            // onToggle(id, !isActive);
        } catch (error) {
            console.error('Error toggling event key:', error);
        } finally {
            setLoading(false);
        }
    };

    return (
        <span  style={{ display: 'flex', marginTop: '1rem', alignItems: 'center', justifyContent: 'space-between' }}>
              {show && (
                <>
            <Description title={name} content={description} />
            <div>
                {/* Delete Button */}
                <Button type="error" auto scale={2 / 4} onClick={() => handleDelete(id)} placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}>Delete</Button>
            <Button 
                            style={{ borderRadius: 2 }}
                            icon={isActive ? <PauseFill /> : <PlayFill />}
                            auto
                            scale={2 / 4}
                            onClick={handleAction}
                            loading={loading}
                            type={isActive ? "secondary" : "success"} placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}            >
                {isActive ? "Deactivate" : "Activate"}
            </Button>
            </div>
            </>
            )}
        </span>
    );
};

export default ActionCard;