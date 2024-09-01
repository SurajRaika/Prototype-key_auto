import { Pause, SkipBack, SkipForward } from "@geist-ui/icons";
import { Textarea, Button } from "@geist-ui/core";
import { useState, useEffect, useRef } from "react";
import RecordPausePlayButton from "../RecordPausePlayButton";
import { Event, listen } from '@tauri-apps/api/event';
import { ParallelActions } from "../../types";
interface KeyEditorProps {
    value: ParallelActions[];
    onChange: (value: ParallelActions[] | ((prevKeys: ParallelActions[]) => ParallelActions[])) => void;
}

const KeyEditor: React.FC<KeyEditorProps> = ({ value, onChange }) => {

    const [isRecording, setIsRecording] = useState<boolean>(false);
    // const [value, onChange] = useState<ParallelActions[]>([]);
    const unlisten = useRef<(() => void) | null>(null);

    useEffect(() => {
        async function setupListener() {
            if (isRecording && !unlisten.current) {
                unlisten.current = await listen<String>('updateCounter', (event: Event<String>) => {
                    const keyPresses: ParallelActions = JSON.parse(event.payload.toString());
                    onChange((prevKeys:ParallelActions[]) => [...prevKeys, keyPresses]);
                });
            } else if (!isRecording && unlisten.current) {
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
    }, [isRecording]);

    const handleTextChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
        const newValue = event.target.value;
        try {
            const newActionKeys = JSON.parse(newValue);
            if (Array.isArray(newActionKeys)) {
                onChange(newActionKeys);
                onChange(value)

            }
        } catch (e) {
            console.error('Invalid JSON input:', e);
        }

    };

    const handleClear = () => {
        onChange([]);
    };

    return (
        <div style={{ display: "flex", flexDirection: 'column', gap: '1rem' }}>
            <span style={{ display: "flex", justifyContent: "space-between" }}>
                <Button ghost icon={<SkipBack />} auto scale={2 / 4} placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}></Button>
                <RecordPausePlayButton isRecording={isRecording} setIsRecording={setIsRecording} />
                <Button ghost icon={<SkipForward />} auto scale={2 / 4} placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}></Button>
            </span>
            {/* <div  */}
            <Textarea
                type="secondary"
                // height={"100%"}
                style={{ aspectRatio: 16 / 6 }}
                className="highlight"
                width="100%"
                value={JSON.stringify(value, null, 2)}
                onChange={handleTextChange}
                placeholder="Action Keys Editor" onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}            />
            {/* </div> */}
            <Button onClick={handleClear} placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}>Clear</Button>
        </div>
    );
}

export default KeyEditor;