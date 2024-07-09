import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Box, Button, List, ListItem, ListItemText, Typography, Tooltip, IconButton } from '@mui/material';
import RefreshIcon from '../assets/reload';
import ClearIcon from '../assets/clear';
import UserSync from '../assets/users'

import { Match, SearchResult } from "../types";
import SyncLog from './syncLog';

const DeviceList: React.FC = () => {
    const [devices, setDevices] = useState<Match[]>([]);
    const [lastRefreshed, setLastRefreshed] = useState<Date>(new Date());
    const [logs, setLogs] = useState<string[]>([]);
    const [remainingTime, setRemainingTime] = useState<number>(0);
    const [forceRefresh, setForceRefresh] = useState<boolean>(false);

    const updateLastRefreshed = () => {
        setLastRefreshed(new Date());
    };

    const nextRefreshTime = new Date(lastRefreshed.getTime() + 10 * 60 * 1000);

    const calculateRemainingTime = () => {
        const currentTime = new Date();
        const timeDifference = nextRefreshTime.getTime() - currentTime.getTime();
        return timeDifference > 0 ? timeDifference : 0;
    };

    useEffect(() => {
        const interval = setInterval(() => {
            setRemainingTime(calculateRemainingTime());
        }, 1000);

        return () => clearInterval(interval);
    }, [lastRefreshed]);

    useEffect(() => {
        fetchDevices();
        const interval = setInterval(() => {
            fetchDevices();
        }, 10 * 60 * 1000); // 10 minutes

        return () => clearInterval(interval);
    }, [forceRefresh]);

    const fetchDevices = async () => {
        try {
            const response: JSON = await invoke('get_all_devices', { companyKey: '' });
            let data: { 'SearchResult': SearchResult } = JSON.parse(`${response}`);

            setDevices(data.SearchResult.MatchList);

            let currentDate = new Date();
            let formattedDate = currentDate.toLocaleString();

            let newLogs = data.SearchResult.MatchList.map((match) => {
                return `${match.Device.devName} has been identified ${match.Device.devStatus} - ${formattedDate}`;
            });

            setLogs(prevLogs => [...prevLogs, ...newLogs]);
            updateLastRefreshed();
        } catch (error) {
            console.error('Error fetching devices:', error);
            let currentDate = new Date();
            let formattedDate = currentDate.toLocaleString();
            setLogs(prevLogs => [...prevLogs, `Error syncing: ${error} - ${formattedDate}`]);
        }
    };

    const syncDevices = async () => {
        for (let match of devices) {
            try {
                const logs: [] = await invoke('fetch_and_upload_data', { deviceId: match.Device.devIndex });
                console.log(logs)
                setLogs(prevLogs => [...prevLogs, ...logs]);
            } catch (error) {
                console.log(error, 'error')
                setLogs(prevLogs => [...prevLogs, 'Error updating data for device ' + match.Device.devName]);
            }
        }
    };

    const syncDevice = async (devName: string) => {
        try {
            const result = `${devName} synced successfully.`;
            setLogs(prevLogs => [...prevLogs, result]);
        } catch (error) {
            console.error('Error syncing device:', error);
            setLogs(prevLogs => [...prevLogs, `Error syncing ${devName}: ${error}`]);
        }
    };

    const syncUsers = async () => {
        if (devices.length) {
            try {
                for (let match of devices) {
                    try {
                        const logs: [] = await invoke('fetch_and_upload_users_data', { deviceId: match.Device.devIndex }); // Adjust the parameters as needed
                        console.log(logs);
                        setLogs(prevLogs => [...prevLogs, ...logs]);
                    } catch (error) {
                        console.log(error, 'error')
                        setLogs(prevLogs => [...prevLogs, 'Error updating users for device ' + match.Device.devName]);
                    }
                }
            } catch (error) {
                console.error('Error syncing users:', error);
                setLogs(prevLogs => [...prevLogs, `Error syncing users: ${error}`]);
            }
        }
        else {
            setLogs(prevLogs => [...prevLogs, 'You need at least one device to get results from.']);
        }
    };

    const getStatusColor = (status: string) => {
        return status === 'online' ? 'green' : 'red';
    };

    const clearLogs = () => {
        setLogs([]);
    };

    useEffect(() => {
        syncDevices();
    }, [devices]);

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                Device List
            </Typography>
            <List style={{ paddingInline: '5vh' }}>
                {devices.map((match, index) => (
                    <ListItem key={index} disablePadding>
                        <ListItemText
                            primary={
                                <Box display="flex" alignItems="center">
                                    <strong>{match.Device.devName}</strong>&nbsp;
                                    <span style={{ color: getStatusColor(match.Device.devStatus) }}>
                                        {match.Device.devStatus.toUpperCase()}
                                    </span>
                                </Box>
                            }
                            secondary={`Type: ${match.Device.devType}, Version: ${match.Device.devVersion}`}
                        />
                        <Button disabled variant="contained" color="primary" onClick={() => syncDevice(match.Device.devName)}>
                            Sync
                        </Button>
                    </ListItem>
                ))}
            </List>

            <Box display="flex" justifyContent="flex-end" style={{ marginBottom: '-4rem', marginRight: '2rem', justifyContent: 'space-evenly' }} >
                <div>
                    <Tooltip title="Refresh Now">
                        <IconButton 
                            color="primary" 
                            onClick={() => setForceRefresh(!forceRefresh)}
                        >
                            <RefreshIcon />
                        </IconButton>
                    </Tooltip>
                    <Tooltip title="Sync Users">
                        <IconButton 
                            color="secondary" 
                            onClick={syncUsers}
                        >
                            <UserSync />
                        </IconButton>
                    </Tooltip>
                </div>

                <Tooltip title="Clear Logs">
                    <IconButton 
                        color="warning" 
                        onClick={clearLogs}
                    >
                        <ClearIcon />
                    </IconButton>
                </Tooltip>

            </Box>

            <SyncLog logs={logs} lastRefreshed={lastRefreshed} remainingTime={remainingTime} />
        </Box>
    );
};

export default DeviceList;
