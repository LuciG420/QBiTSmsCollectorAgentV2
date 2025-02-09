import React, { useState } from 'react';
import { TextInput, Button, FlatList, StyleSheet, View, Text } from 'react-native';
import axios from 'axios';

const App = () => {
    const [config, setConfig] = useState('');
    const [response, setResponse] = useState('');

    const loadConfig = async () => {
        try {
            const res = await axios.post('http://localhost:2019/load', JSON.parse(config));
            setResponse('Configuration loaded successfully!');
        } catch (error) {
            setResponse('Failed to load configuration');
        }
    };

    return (
        <View style={styles.container}>
            <TextInput
                style={styles.input}
                value={config}
                onChangeText={setConfig}
                placeholder="Enter Caddy Configuration JSON"
                multiline
            />
            <Button title="Load Configuration" onPress={loadConfig} />
            <Text style={styles.response}>{response}</Text>
        </View>
    );
};

const styles = StyleSheet.create({
    container: { flex: 1, padding: 16 },
    input: { height: 100, borderColor: 'gray', borderWidth: 1, marginBottom: 12, paddingHorizontal: 8 },
    response: { marginTop: 12, fontSize: 16 }
});

export default App;
