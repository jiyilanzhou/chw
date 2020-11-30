
/*
0. The Subkey Tool  子项工具

1. Installation    安装
    a. Build from Source    从源构建
    b. Compiling with Cargo 用 Cargo 编译

2. Generating Keys  产生密钥
    a. Secret phrase (aka "mnemonic phrase")            秘密短语（又名"助记短语"）
    b. Secret Seed (aka "Private Key" or "Raw Seed")    秘密种子（又名"私钥"或"原始种子"）
    c. Public Key (aka "Account ID")                    公用密钥（又名"帐户ID"）
    d. SS58 Address (aka "Public Address")              SS58地址（又名"公共地址"）
    // 当前 Subkey 支持以下加密密钥对和签名算法：
        sr25519: Schorr signatures on the Ristretto group
        ed25519: ed25519
        secp256k1: ECDSA signatures on secp256k1

3. Password Protected Keys  密码保护的钥匙

4. Inspecting Keys 检查钥匙

5. Inserting Keys to a Node's Keystore  将密钥插入节点的密钥库

6. HD Derivation
    a. Hard Key Derivation      硬键派生
    b. Soft Key Derivation      软键派生
    c. Putting it All Together  混合放置

7. Well-Known Keys 知名键

8. Signing and Verifying Messages   签名和验证消息
    a. Signing      签名
    b. Verifying    验证

9. Generating Node Keys 生成节点密钥

10. More Subkey to Explore  更多子项可探索

 */