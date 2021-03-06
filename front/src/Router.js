import React from "react";
import DevTools from './containers/DevTools';
import { Provider } from 'react-redux';
import { ConnectedRouter } from 'connected-react-router';
import {Route} from "react-router-dom";
import { history } from './store';
import Home from './components/Home';
import Menu from './components/Menu';
import Bids from './components/Bids';
import SignupForm from './components/forms/signup';
import SigninForm from './components/forms/signin';
import ProductForm from "./components/forms/product";


const Root = ({ store }) => (
  <Provider store={store}>
    <ConnectedRouter history={history}>
      <Menu />
      <Route path="/home" exact component={Home} />
      <Route path="/bids" exact component={Bids} />
      <Route path="/signin" exact component={SigninForm} />
      <Route path="/signup" exact component={SignupForm} />
      <Route path="/product" exact component={ProductForm} />
      <DevTools />
    </ConnectedRouter>
  </Provider>
)

export default Root;
