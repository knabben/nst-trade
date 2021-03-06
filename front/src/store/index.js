import createRootReducer from '../reducers'
import DevTools from '../containers/DevTools'
import { createStore, applyMiddleware, compose } from 'redux'
import { routerMiddleware } from 'connected-react-router'
import { createBrowserHistory } from 'history'

import { loggerMiddleware } from '../middleware/logger';
import { apiMiddleware } from '../middleware/api';
import { userMiddleware } from '../middleware/user';
import { bidMiddleware } from '../middleware/bid';
import { productMiddleware } from '../middleware/product';
import { notificationMiddleware } from '../middleware/notifications';
import { routerDispatchMiddleware } from '../middleware/router';

export const history = createBrowserHistory()

const featureMiddleware = [
  routerDispatchMiddleware,
  userMiddleware,
  productMiddleware,
  bidMiddleware,
]

const coreMiddleware = [
  apiMiddleware,
  notificationMiddleware,
  loggerMiddleware
]

const configureStore = preloadedState => {
  const store = createStore(
    createRootReducer(history),
    preloadedState,
    compose(
      applyMiddleware(
        routerMiddleware(history), 
        ...featureMiddleware,
        ...coreMiddleware
      ),
      DevTools.instrument()
    )
  )

  if (module.hot) {
    // Enable Webpack hot module replacement for reducers
    module.hot.accept('../reducers', () => {
      store.replaceReducer(createRootReducer(history))
    })
  }

  return store
}

export default configureStore