import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.cm as cm
import matplotlib.patches as mpatches
import numpy as np
import os

def create_filename_base(prefix, lambda_val, j_val, mesh_x, mesh_y, div, threshold_berry, dir = "data_qmd"):
    """
    設定値からファイル名のベース部分を生成する関数
    """
    # floatを文字列に変換し、'.'を'p'に置換
    lambda_str = str(lambda_val).replace('.', 'p')
    j_str = str(j_val).replace('.', 'p')

    # f-stringを使って最終的な文字列を組み立てる
    filename = (
        f"./{dir}/data_{prefix}_lambda{lambda_str}_j{j_str}_mesh_x{mesh_x}_mesh_y{mesh_y}_div{div}_thresh10em{threshold_berry}.dat"
    )
    
    return filename

def create_filename_base_2(prefix, lambda_val, j_val, mesh_x, mesh_y, div, threshold_berry, main_mesh, dir = "data_qmd"):
    """
    設定値からファイル名のベース部分を生成する関数
    """
    # floatを文字列に変換し、'.'を'p'に置換
    lambda_str = str(lambda_val).replace('.', 'p')
    j_str = str(j_val).replace('.', 'p')

    # f-stringを使って最終的な文字列を組み立てる
    filename = (
        f"./{dir}/data_{prefix}_lambda{lambda_str}_j{j_str}_mesh_x{mesh_x}_mesh_y{mesh_y}_div{div}_thresh10em{threshold_berry}_main_mesh{main_mesh}.dat"
    )
    
    return filename



lam = 0.3
j = 0.1
mesh = 400
div = 307
threshold_berry = 12
main_mesh = 35

only_norm = False

if only_norm:
    nrows = 3
else:
    nrows = 5

spin_names = ["FmTmd","One1Tmd","One2Tmd","TwinTmd","Tri1Tmd","UuudddTmd","Tri2Tmd","SatoTmd","stable"]

file_list = [
    create_filename_base_2(spin_names[0], lam, j, mesh, mesh, div, threshold_berry, main_mesh,"compare_6_spinmodel"),
    create_filename_base_2(spin_names[1], lam, j, mesh, mesh, div, threshold_berry, main_mesh,"compare_6_spinmodel"),
    create_filename_base_2(spin_names[2], lam, j, mesh, mesh, div, threshold_berry, main_mesh,"compare_6_spinmodel"),
    create_filename_base_2(spin_names[3], lam, j, mesh, mesh, div, threshold_berry, main_mesh,"compare_6_spinmodel"),
    create_filename_base_2(spin_names[4], lam, j, mesh, mesh, div, threshold_berry, main_mesh,"compare_6_spinmodel"),
    create_filename_base_2(spin_names[5], lam, j, mesh, mesh, div, threshold_berry, main_mesh,"compare_6_spinmodel"),
    create_filename_base_2(spin_names[6], lam, j, mesh, mesh, div, threshold_berry, main_mesh,"compare_6_spinmodel"),
    create_filename_base_2(spin_names[7], lam, j, mesh, mesh, div, threshold_berry, main_mesh,"compare_6_spinmodel"),
    create_filename_base_2(spin_names[8], lam, j, mesh, mesh, div, threshold_berry, main_mesh,"compare_6_spinmodel"),
]

labels = ["FM"   ,"UDDDDD"     ,"DUDDDD"    ,"UUDDDD"    ,"UDUDDD"    ,"UUUDDD"    ,"DUDUDD"    ,"AFM"     ,"Most stable"]
colors = cm.gist_rainbow(np.linspace(0, 1, 8))

out_title = f"stable_lambda{lam}_j{j}_rev2_{only_norm}"

# ==== 色設定 ====
num_files = len(file_list) -1


# ==== matplotlib全体のフォントサイズ設定 ====
plt.rcParams['font.size'] = 24

# ==== プロット準備 ====
fig, axes = plt.subplots(nrows=nrows, ncols=1, figsize=(12, 20), sharex=True)  # 横幅を広げた

# ==== 全ファイルのy軸範囲をまとめて取得 ====
ymins = np.full(5, np.inf)
ymaxs = np.full(5, -np.inf)

# 全データを読み込み
dfs = []
for i, filename in enumerate(file_list[:-1]):  # stableを除く
    file_path = f"{filename}"
    df = pd.read_csv(file_path, comment='#', header=None,
                     names=["n","energy", "bc_sum", "bcd_x_sum", "bcd_y_sum", "qmd_x_sum", "qmd_y_sum"])
    dfs.append(df)

stable_df = pd.read_csv(file_list[-1], comment='#', header=None,
                     names=["n","energy", "bc_sum", "bcd_x_sum", "bcd_y_sum", "qmd_x_sum", "qmd_y_sum", "stable"])

# 線グラフをプロット（stableを除く）
for i in range(len(file_list) - 1):  # stableを除く
    color = colors[i]
    df = dfs[i]
    line_width = 0.4

    if labels[i] == "UUUDDD" or labels[i] == "UUDDDD":
        line_width = 1

    axes[0].plot(df["n"], df["bc_sum"], color=color, linewidth=line_width)

    if only_norm is False:
        axes[1].plot(df["n"], df["bcd_x_sum"], color=color, linewidth=line_width)
        axes[2].plot(df["n"], df["bcd_y_sum"], color=color, linewidth=line_width)
        axes[3].plot(df["n"], df["qmd_x_sum"], color=color, linewidth=line_width)
        axes[4].plot(df["n"], df["qmd_y_sum"], color=color, linewidth=line_width)
    else:
        axes[1].plot(df["n"], np.sqrt(df["bcd_x_sum"] ** 2 + df["bcd_y_sum"] ** 2), color=color, linewidth=line_width)
        axes[2].plot(df["n"], np.sqrt(df["qmd_x_sum"] ** 2 + df["qmd_y_sum"] ** 2), color=color, linewidth=line_width)


# 各stableのデータポイントに対して処理
    
colors_for_stable = []

for stable in stable_df["stable"]:
    if stable is None:
        colors_for_stable.append('black')  # stableがNoneの場合は黒色
    elif stable == spin_names[0]:
        colors_for_stable.append(colors[0])
    elif stable == spin_names[1]:
        colors_for_stable.append(colors[1])
    elif stable == spin_names[2]:
        colors_for_stable.append(colors[2])
    elif stable == spin_names[3]:
        colors_for_stable.append(colors[3])
    elif stable == spin_names[4]:
        colors_for_stable.append(colors[4])
    elif stable == spin_names[5]:
        colors_for_stable.append(colors[5])
    elif stable == spin_names[6]:
        colors_for_stable.append(colors[6])
    elif stable == spin_names[7]:
        colors_for_stable.append(colors[7])
    else:
        colors_for_stable.append('black')  # その他の場合は黒色
    
marker_size = 25

axes[0].scatter(stable_df["n"], stable_df["bc_sum"], color=colors_for_stable, s=marker_size, alpha=0.7)

if only_norm is False:
    axes[1].scatter(stable_df["n"], stable_df["bcd_x_sum"], color=colors_for_stable, s=marker_size, alpha=0.7)
    axes[2].scatter(stable_df["n"], stable_df["bcd_y_sum"], color=colors_for_stable, s=marker_size, alpha=0.7)
    axes[3].scatter(stable_df["n"], stable_df["qmd_x_sum"], color=colors_for_stable, s=marker_size, alpha=0.7)
    axes[4].scatter(stable_df["n"], stable_df["qmd_y_sum"], color=colors_for_stable, s=marker_size, alpha=0.7)
else:
    axes[1].scatter(stable_df["n"], np.sqrt(stable_df["bcd_x_sum"] ** 2 + stable_df["bcd_y_sum"] ** 2), color=colors_for_stable, s=marker_size, alpha=0.7)
    axes[2].scatter(stable_df["n"], np.sqrt(stable_df["qmd_x_sum"] ** 2 + stable_df["qmd_y_sum"] ** 2), color=colors_for_stable, s=marker_size, alpha=0.7)

# y軸範囲の更新
ymins[0] = min(ymins[0], stable_df["bc_sum"].min())
ymaxs[0] = max(ymaxs[0], stable_df["bc_sum"].max())
if only_norm is False:
    ymins[1] = min(ymins[1], stable_df["bcd_x_sum"].min())
    ymaxs[1] = max(ymaxs[1], stable_df["bcd_x_sum"].max())
    ymins[2] = min(ymins[2], stable_df["bcd_y_sum"].min())
    ymaxs[2] = max(ymaxs[2], stable_df["bcd_y_sum"].max())
    ymins[3] = min(ymins[3], stable_df["qmd_x_sum"].min())
    ymaxs[3] = max(ymaxs[3], stable_df["qmd_x_sum"].max())
    ymins[4] = min(ymins[4], stable_df["qmd_y_sum"].min())
    ymaxs[4] = max(ymaxs[4], stable_df["qmd_y_sum"].max())
else:
    ymins[1] = min(ymins[1], np.sqrt(stable_df["bcd_x_sum"] ** 2 + stable_df["bcd_y_sum"] ** 2).min())
    ymaxs[1] = max(ymaxs[1], np.sqrt(stable_df["bcd_x_sum"] ** 2 + stable_df["bcd_y_sum"] ** 2).max())
    ymins[2] = min(ymins[2], np.sqrt(stable_df["qmd_x_sum"] ** 2 + stable_df["qmd_y_sum"] ** 2).min())
    ymaxs[2] = max(ymaxs[2], np.sqrt(stable_df["qmd_x_sum"] ** 2 + stable_df["qmd_y_sum"] ** 2).max())


# ==== y軸範囲設定 ====
for ax, ymin, ymax in zip(axes, ymins, ymaxs):
    yrange = (ymax - ymin) * 1.1
    if yrange < 2e-5:
        ax.set_ylim(-1e-5, 1e-5)
    elif yrange > 500:
        ax.set_ylim(-250, 250)
    else:
        ax.set_ylim(-abs(yrange/2), abs(yrange/2))

    ax.set_xlim(0.0,1.0)

# ==== ラベルやタイトル ====
# axes[0].set_ylabel("BC")
# if only_norm is False:
#     axes[1].set_ylabel("BCD X")
#     axes[2].set_ylabel("BCD Y")
#     axes[3].set_ylabel("QMD X")
#     axes[4].set_ylabel("QMD Y")
#     axes[4].set_xlabel("n")
# else:
#     axes[1].set_ylabel("BCD")
#     axes[2].set_ylabel("QMD")
#     axes[2].set_xlabel("n")

for ax in axes:
    ax.grid(True)

# ==== カラーパッチと凡例 ====
import matplotlib.patches as mpatches
patches = [mpatches.Patch(color=colors[i], label=str(labels[i])) for i in range(num_files)]
fig.legend(
    handles=patches,
    title="",
    loc='lower center',
    ncol=4,  # 一段表示
    frameon=False,
    fontsize=20,
    title_fontsize=22
)

# ==== 凡例とタイトルに余白を取る ====
plt.tight_layout(rect=[0, 0.14, 1, 0.93])  # 下にスペース追加

# ==== 保存 ====
plt.savefig(f"./figure_qmd/{out_title}.png", dpi=300)
print(f"画像を保存しました")

plt.show()

