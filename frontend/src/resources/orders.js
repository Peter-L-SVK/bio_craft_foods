import * as React from 'react';
import {
    List,
    Datagrid,
    TextField,
    Edit,
    Create,
    SimpleForm,
    NumberInput,
    DateInput,
    Show,
    SimpleShowLayout,
    BulkDeleteButton,
} from 'react-admin';
import { required, number } from 'react-admin';

const OrderBulkActionButtons = (props) => (
    <React.Fragment>
        <BulkDeleteButton {...props} />
    </React.Fragment>
);

export const OrderList = () => (
    <List>
        <Datagrid rowClick="edit" bulkActionButtons={<OrderBulkActionButtons />}>
            <TextField source="id" />
            <TextField source="customer_id" />
            <TextField source="product_id" />
            <TextField source="quantity" />
            <TextField source="order_date" />
        </Datagrid>
    </List>
);


export const OrderCreate = () => (
    <Create>
        <SimpleForm>
            <NumberInput source="customer_id" validate={[required(), number()]} /> {/* Use NumberInput */}
            <NumberInput source="product_id" validate={[required(), number()]} />  {/* Use NumberInput */}
            <NumberInput source="quantity" validate={[required(), number()]} />
            <DateInput source="order_date" validate={[required()]} />
        </SimpleForm>
    </Create>
);

export const OrderEdit = () => (
    <Edit>
        <SimpleForm>
            <NumberInput source="customer_id" validate={[required(), number()]} /> {/* Use NumberInput */}
            <NumberInput source="product_id" validate={[required(), number()]} />  {/* Use NumberInput */}
            <NumberInput source="quantity" validate={[required(), number()]} />
            <DateInput source="order_date" validate={[required()]} />
        </SimpleForm>
    </Edit>
);

export const OrderShow = () => (
    <Show>
        <SimpleShowLayout>
            <TextField source="id" />
            <TextField source="customer_id" />
            <TextField source="product_id" />
            <TextField source="quantity" />
            <TextField source="order_date" />
        </SimpleShowLayout>
    </Show>
);