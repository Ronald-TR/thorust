import * as React from 'react';
import Container from '@mui/material/Container';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';
import NodeTestTable from '@/components/NodeTestTable';

export default function TasksPage() {
  return (
    <Container>
      <Box
        sx={{
          display: 'flex',
          flexDirection: 'column',
          justifyContent: 'center',
          alignItems: 'center',
        }}
      >
        <Typography variant="body1" gutterBottom>
          Tasks Page
        </Typography>
        <NodeTestTable />
      </Box>
    </Container>
  );
}
