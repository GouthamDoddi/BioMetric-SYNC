import React from 'react';
import { Box, Typography } from '@mui/material';

interface SyncLogProps {
    logs: string[],
    lastRefreshed: Date,
    remainingTime: number,
}



const SyncLog: React.FC<SyncLogProps> = ({ logs, lastRefreshed, remainingTime }) => {
    const formatTimeForRefresh = (milliseconds: number) => {
        const totalSeconds = Math.floor(milliseconds / 1000);
        const minutes = Math.floor(totalSeconds / 60);
        const seconds = totalSeconds % 60;
        return `${minutes}m ${seconds}s`;
    };

    return (
        <Box mt={4}>
            <Typography variant="h5" gutterBottom>
                Sync Logs
            </Typography>


            <Box bgcolor="black" color="white" p={2} style={{ fontFamily: 'monospace', maxHeight: '200px', overflowY: 'auto' }}>
                <div style={{ display:'flex', color: 'white', flexDirection: 'row', padding: '2px', justifyContent: 'space-between'}}>
                    <p>Last refreshed: {lastRefreshed.toLocaleString()}</p>
                    <p>Next refresh in: {formatTimeForRefresh(remainingTime)}</p>
                </div>
                {logs.map((log, index) => (

                    <Typography key={index} variant="body2" gutterBottom>
                        {log}
                    </Typography>
                ))}
            </Box>
        </Box>
    );
};

export default SyncLog;
