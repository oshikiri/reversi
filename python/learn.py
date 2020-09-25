#%%
import numpy as np
from scipy.sparse import coo_matrix
import pandas as pd
from sklearn.linear_model import SGDRegressor

histories = np.loadtxt("./data/game-histories/02.csv", delimiter=",", dtype="i4")

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
i1 = np.arange(histories.shape[0] * 11)

i2_vstacked = histories[:, 3:]
offsets = np.hstack([[0], (3 ** np.array(n_cells_each_pattern[:10])).cumsum()])
i2 = np.ravel(i2_vstacked + offsets)

X = coo_matrix(
  (np.ones_like(i1), (i1, i2)),
  shape=(i1.max() + 1, xcol)
)
y = np.repeat(histories[:, 1], 11)

# %%
model = SGDRegressor(penalty = 'l2')
model.fit(X, y)

# %%
df_params = pd.DataFrame({
  'pattern_id': np.repeat(pattern_ids, 3 ** np.array(n_cells_each_pattern)),
  'pattern_index': np.hstack([np.arange(n) for n in 3 ** np.array(n_cells_each_pattern)]),
  'coef': model.coef_
})
df_params.to_csv('data/parameters/0925.csv', index = False)

df_params.sort_values('coef')

# %%
def base_10_to_base_n(x, n):
    if int(x / n):
        return base_10_to_base_n(int(x / n), n) + str(x % n)
    return str(x % n)

base_10_to_base_n(189, 3)
