import * as React from 'react';
import { List, Datagrid, TextField, Edit, Create, SimpleForm, TextInput, NumberInput, BooleanInput, Show, Filter, SearchInput, SimpleShowLayout } from 'react-admin';
import { required, number } from 'react-admin';

const ProductFilter = (props) => (
    <Filter {...props}>
        <SearchInput source="q" alwaysOn />
    </Filter>
);

export const ProductList = () => (
    <List filters={<ProductFilter />} onSuccess={(data) => console.log("Fetched data in ProductList:", data)}>
        <Datagrid rowClick="edit">
            <TextField source="id" />
            <TextField source="name" />
            <TextField source="description" />
            <TextField source="price" />
            <TextField source="in_stock" />
        </Datagrid>
    </List>
);

export const ProductCreate = () => (
    <Create>
        <SimpleForm>
            <TextInput source="name" validate={[required()]} />
            <TextInput source="description" />
            <NumberInput source="price" validate={[required(), number()]} />
            <BooleanInput source="in_stock" />
        </SimpleForm>
    </Create>
);

export const ProductEdit = () => (
    <Edit>
        <SimpleForm>
            <TextInput source="name" validate={[required()]} />
            <TextInput source="description" />
            <NumberInput source="price" validate={[required(), number()]} />
            <BooleanInput source="in_stock" />
        </SimpleForm>
    </Edit>
);

export const ProductShow = () => (
    <Show>
        <SimpleShowLayout>
            <TextField source="id" />
            <TextField source="name" />
            <TextField source="description" />
            <TextField source="price" />
            <TextField source="in_stock" />
        </SimpleShowLayout>
    </Show>
);