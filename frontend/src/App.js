import * as React from 'react';
import { Admin, Resource } from 'react-admin';
import dataProvider from './dataProvider'; // Import the custom data provider
import { ProductList, ProductEdit, ProductCreate, ProductShow } from './resources/products';
import { OrderList, OrderEdit, OrderCreate, OrderShow } from './resources/orders';
import { CustomerList, CustomerEdit, CustomerCreate, CustomerShow } from './resources/customers';

const App = () => (
    <Admin dataProvider={dataProvider}>
        <Resource
            name="products"
            list={ProductList}
            edit={ProductEdit}
            create={ProductCreate}
            show={ProductShow}
        />
        <Resource
            name="orders"
            list={OrderList}
            edit={OrderEdit}
            create={OrderCreate}
            show={OrderShow}
        />
        <Resource
	    name="customers"
    	    list={CustomerList}
    	    edit={CustomerEdit}
    	    create={CustomerCreate}
    	    show={CustomerShow}
    	    />
    	    </Admin>
    	    );

export default App;