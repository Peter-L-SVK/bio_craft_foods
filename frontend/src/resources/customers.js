import * as React from 'react';
import { List, Datagrid, TextField, Edit, Create, SimpleForm, TextInput, Show, SimpleShowLayout, BulkDeleteButton } from 'react-admin';
import { required, email } from 'react-admin';

export const CustomerList = () => (
    <List>
        <Datagrid rowClick="edit">
            <TextField source="id" />
            <TextField source="name" />
            <TextField source="email" />
            <TextField source="address" />
        </Datagrid>
    </List>
);

export const CustomerCreate = () => (
    <Create>
        <SimpleForm>
            <TextInput source="name" validate={[required()]} />
            <TextInput source="email" validate={[required(), email()]} />
            <TextInput source="address" />
        </SimpleForm>
    </Create>
);

export const CustomerEdit = () => (
    <Edit>
        <SimpleForm>
            <TextInput source="name" validate={[required()]} />
            <TextInput source="email" validate={[required(), email()]} />
            <TextInput source="address" />
        </SimpleForm>
    </Edit>
);

export const CustomerShow = () => (
    <Show>
        <SimpleShowLayout>
            <TextField source="id" />
            <TextField source="name" />
            <TextField source="email" />
            <TextField source="address" />
        </SimpleShowLayout>
    </Show>
);