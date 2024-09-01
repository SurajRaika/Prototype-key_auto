import React, { useEffect, useState } from 'react';
import { Link } from "react-router-dom";
import { useAppSelector, useAppDispatch } from '../../../hooks';
import { increment, decrement } from '../../../features/eventSlice';
import ActionCard from '../../ActionCard';
import "./style.css";
import { Description, Button, Text, Card, Divider } from "@geist-ui/core";
import { PlusSquare, ChevronsLeft, Pause, PlayFill, MinusSquare, RefreshCw } from "@geist-ui/icons";
import { readTextFile } from '@tauri-apps/api/fs';
import { appLocalDataDir } from '@tauri-apps/api/path';
import { join } from '@tauri-apps/api/path';

import { EventKey, EventKeys } from '../../../types';

async function loadEventKeys(): Promise<EventKeys> {
    try {
        const localDataDir = await appLocalDataDir();
        const filePath = await join(localDataDir, 'EventKeys.json');
        const fileContents = await readTextFile(filePath);
        const eventKeys: EventKeys = JSON.parse(fileContents);
        console.log(eventKeys);

        return eventKeys;
    } catch (error) {
        console.error('Error loading EventKeys:', error);
        throw error;
    }
}

function Home() {
    const [eventKeys, setEventKeys] = useState<EventKeys>([]);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const loadData = async () => {
        setIsLoading(true);
        setError(null);
        try {
            const keys = await loadEventKeys();
            setEventKeys(keys);
        } catch (error) {
            setError('Failed to load EventKeys');
            console.error('Failed to load EventKeys:', error);
        } finally {
            setIsLoading(false);
        }
    };

    useEffect(() => {
        loadData();
    }, []);

    return (
        <div className="container">
            <div style={{ display: 'flex', marginBottom: '1rem', alignContent: 'center', justifyContent: 'space-between' }}>
                <Button
                    style={{ borderRadius: 2 }}
                    icon={<RefreshCw />}
                    auto
                    scale={2 / 4}
                    onClick={loadData}
                    loading={isLoading} placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}                >
                    Refresh
                </Button>
                <Link to="/NewAction">
                    <Button style={{ borderRadius: 2 }} icon={<PlusSquare />} auto scale={2 / 4} type="secondary" placeholder={undefined} onPointerEnterCapture={undefined} onPointerLeaveCapture={undefined}> New Action </Button>
                </Link>
            </div>
            <Divider />

            {error && <Text type="error">{error}</Text>}

            {eventKeys.map((eventKey) => (
                <React.Fragment key={eventKey.id}>
                    <ActionCard
                        eventKey={eventKey}
                        name={eventKey.name}
                        description={eventKey.description}
                        id={eventKey.id}
                    />
                    <Divider />
                </React.Fragment>
            ))}

            {eventKeys.length === 0 && !isLoading && (
                <Text>No event keys found. Try refreshing or adding new actions.</Text>
            )}
        </div>
    );
}

export default Home;
