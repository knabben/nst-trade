// action types
export const SET_LOADER = 'LOADER';

export const setLoader = ({state, feature}) => ({
  type: `${feature} ${SET_LOADER}`,
  payload: state,
  meta: {
    feature
  }
});