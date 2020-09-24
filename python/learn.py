#%%
import numpy as np
from scipy.sparse import coo_matrix

# %%
histories = np.loadtxt("./data/game-histories/02.csv", delimiter=",", dtype="i4")

# %%
y = histories[:, 1]

n_cells_each_pattern = [4, 5, 6, 7, 8, 8, 8, 8, 10, 10, 9]
n_patterns = len(n_cells_each_pattern)
xcol = (3 ** np.array(n_cells_each_pattern)).sum()

i1 = np.repeat(histories[:10, 1], n_patterns)

i2_vstacked = histories[:10, 3:]
offsets = np.hstack([[0], (3 ** np.array(n_cells_each_pattern[:10])).cumsum()])
i2 = np.ravel(i2_vstacked + offsets)


# %%
ones = np.ones_like(i1)
X = coo_matrix((ones, (i1, i2)), shape=(i1.max() + 1, xcol))
X.count_nonzero()

# %%
