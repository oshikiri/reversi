#%%
import numpy as np
from scipy.sparse import coo_matrix
import pandas as pd

from sklearn.linear_model import SGDRegressor
from sklearn.model_selection import train_test_split

histories = np.loadtxt("./data/game-histories/02.csv", delimiter=",", dtype="i4")
histories.shape

# %%
# 序盤は定石データベースで処理するべきなので除外する
# 終盤は完全読みするので除外する
index = np.reshape(
  np.repeat((histories[: ,1] > 10) & (histories[: ,1] < 50), 47),
  (-1, 47)
)
histories = np.reshape(
  histories[index],
  (-1, 47)
)

# %%
pattern_ids = [i for i in range(11)]
pattern_names = [
  'diag4',
  'diag5',
  'diag6',
  'diag7',
  'diag8',

  'hor./vert.2',
  'hor./vert.3',
  'hor./vert.4',

  'edge+2X',
  '2x5-corner',
  '3x3-corner'
]
n_cells_each_pattern = [4, 5, 6, 7, 8, 8, 8, 8, 10, 10, 9]
n_patterns = len(n_cells_each_pattern)
xcol = (3 ** np.array(n_cells_each_pattern)).sum()

# %%
i1 = np.repeat(
  np.arange(histories.shape[0] * n_patterns),
  4
)
i2_vstacked = histories[:, 3:]
offsets = np.tile(
  np.hstack([[0], (3 ** np.array(n_cells_each_pattern[:10])).cumsum()]),
  4
)
i2 = np.ravel(i2_vstacked + offsets)

X = coo_matrix(
  (np.ones_like(i1), (i1, i2)),
  shape=(i1.max() + 1, xcol)
)
y = np.repeat(histories[:, 2], 11)

X_train, X_test, y_train, y_test_true = train_test_split(X, y, test_size=0.2, random_state=0, shuffle = False)

# %%
model = SGDRegressor(penalty = 'l2')
model.fit(X_train, y_train)

# %%
def base_10_to_base_n(x, n):
    if int(x / n):
        return base_10_to_base_n(int(x / n), n) + str(x % n)
    return str(x % n)

df_params = (
  pd
  .DataFrame({
    'pattern_id': np.repeat(pattern_ids, 3 ** np.array(n_cells_each_pattern)),
    'pattern_index': np.hstack([np.arange(n) for n in 3 ** np.array(n_cells_each_pattern)]),
    'coef': model.coef_
  })
  .assign(
    pattern_name = lambda d: d.pattern_id.map(lambda i: pattern_names[i]),
    pattern_instance = lambda d: d.pattern_index.map(lambda i: base_10_to_base_n(i, 3))
  )
)
df_params.to_csv('data/parameters/0925.csv', index = False)

df_params.sort_values('coef')


# %%
y_test_predict = model.predict(X_test)
(
  pd
  .DataFrame(dict(
    y_test_true = y_test_true,
    y_test_predict = y_test_predict
  ))
  # .sort_values('y_test_true')
)

# %%
from sklearn.metrics import mean_absolute_error
mean_absolute_error(y_test_true, y_test_predict)

# %%
np.savetxt(
  'data/parameters/0925.txt',
  df_params.iloc[:, 2].values,
  fmt='%.3f',
  newline = ','
)
