// import { Box, Collapse, IconButton, TableCell, TableRow, Typography } from "@mui/material";

// const ExpandableTableRow: React.FC<{ device: Device }> = ({ device }) => {
//     const [expanded, setExpanded] = useState(false);

//     const handleExpandClick = () => {
//         setExpanded(!expanded);
//     };

//     return (
//         <>
//             <TableRow>
//                 <TableCell>
//                     <IconButton aria-label="expand row" size="small" onClick={handleExpandClick}>
//                         <ExpandMoreIcon />
//                     </IconButton>
//                 </TableCell>
//                 <TableCell>{device.devName}</TableCell>
//                 <TableCell>{device.devStatus}</TableCell>
//                 <TableCell>{device.devType}</TableCell>
//                 <TableCell>{device.devVersion}</TableCell>
//             </TableRow>
//             <TableRow>
//                 <TableCell style={{ paddingBottom: 0, paddingTop: 0 }} colSpan={5}>
//                     <Collapse in={expanded} timeout="auto" unmountOnExit>
//                         <Box margin={1}>
//                             <Typography variant="h6" gutterBottom component="div">
//                                 More Details
//                             </Typography>
//                             <Typography variant="body1" gutterBottom>
//                                 Additional information about {device.devName}.
//                                 {/* Add more details here */}
//                             </Typography>
//                         </Box>
//                     </Collapse>
//                 </TableCell>
//             </TableRow>
//         </>
//     );
// };

// export default ExpandableTableRow