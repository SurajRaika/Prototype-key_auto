import { Divider, Input, Textarea, Button, useToasts } from "@geist-ui/core";
import KeyEditor from "../../KeyEditor";
import { Link, useNavigate } from "react-router-dom";
import ShortcutKeyInput from "../../EventSelectorInput";
import { ParallelActions, EventKey, EventKeys } from "../../../types";
import { useState } from "react";
import { invoke } from "@tauri-apps/api";
import { appLocalDataDir, join } from "@tauri-apps/api/path";
import { exists, readTextFile, writeTextFile } from "@tauri-apps/api/fs";



function CreateAction() {
    const [name, setName] = useState<string>("");
    const [description, setDescription] = useState<string>("");
    const [event, setEvent] = useState<ParallelActions | null>(null);
    const [actions, setActions] = useState<ParallelActions[]>([]);
    const { setToast } = useToasts();
    const navigate = useNavigate();

    const handleSave = async () => {
        if (!event) {
            setToast({ text: "Event is not set", type: "error" });
            console.error("Event is not set");
            return;
        }

        const newEventKey: EventKey = {
            id: Math.floor(Math.random() * 1000000), // Generate a random number between 0 and 999999
            name,
            description,
            event: [event],
            actions,
        };


        async function SaveNewEventKey(eventKey: EventKey) {
            try {
                const localDataDir = await appLocalDataDir();
                const filePath = await join(localDataDir, 'EventKeys.json');

                if (!(await exists(filePath))) {
                    await writeTextFile(filePath, '[]');
                }

                const fileContents = await readTextFile(filePath);
                const eventKeys: EventKeys = JSON.parse(fileContents);
                // console.log(eventKeys);
                eventKeys.push(eventKey);
                await writeTextFile(filePath, JSON.stringify(eventKeys));
                navigate('/'); // Redirect to home page or list of events

                // return eventKeys;
            } catch (error) {
                console.error('Error loading EventKeys:', error);
                throw error;
            }
        }
        SaveNewEventKey(newEventKey);

        // try {
        //     await invoke('add_active_event_key', { name: JSON.stringify(newEventKey) });
        //     console.log("Successfully saved new event key:", newEventKey);
        //     setToast({ text: "Event key saved successfully!", type: "success" });
        //     navigate('/'); // Redirect to home page or list of events
        // } catch (error) {
        //     console.error("Failed to save event key:", error);
        //     setToast({ text: `Failed to save event key: ${error}`, type: "error" });
        // }




    };

    return (
        <div className="container " style={{ display: "flex", flexDirection: 'column', gap: '1rem' }} >
            <Input
                type="secondary"
                placeholder="Enter Name"
                value={name}
                onChange={(e) => setName(e.target.value)} crossOrigin={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined} />
            <Textarea
                type="secondary"
                placeholder="Please enter a description."
                value={description}
                onChange={(e) => setDescription(e.target.value)} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined} />
            <Divider />
            <ShortcutKeyInput
                value={event}
                onChange={setEvent}
            />
            <KeyEditor
                value={actions}
                onChange={setActions}
            />
            <span style={{ display: "flex", justifyContent: "space-between" }}>
                <Link to="/">
                    <Button type="error" placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}>Cancel</Button>
                </Link>
                <Button type="secondary" onClick={handleSave} placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}>Save</Button>
            </span>
        </div>
    )
}

export default CreateAction;