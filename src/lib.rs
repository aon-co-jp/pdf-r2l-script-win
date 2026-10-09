use lopdf::{Dictionary, Document, Object};
use std::path::Path;

/// 元スクリプト(pdfr2l.py)と同じく、ViewerPreferences の Direction を R2L にし、
/// PageLayout を TwoColumnRight(見開き・右始まり)にして保存する。
/// 既存の ViewerPreferences の他の項目は保持する(元スクリプトは置き換え)。
pub fn convert(src: &Path, dest: &Path) -> Result<(), String> {
    if src == dest {
        return Err("入力と出力が同じです".into());
    }
    let mut doc = Document::load(src).map_err(|e| format!("読み込み失敗: {e}"))?;
    let root_id = doc
        .trailer
        .get(b"Root")
        .and_then(|o| o.as_reference())
        .map_err(|e| format!("Catalogが見つかりません: {e}"))?;

    let vp_ref = doc
        .get_dictionary(root_id)
        .ok()
        .and_then(|c| c.get(b"ViewerPreferences").ok())
        .and_then(|o| o.as_reference().ok());
    let dir = Object::Name(b"R2L".to_vec());
    if let Some(id) = vp_ref {
        doc.get_dictionary_mut(id).map_err(|e| e.to_string())?.set("Direction", dir.clone());
    }
    let catalog = doc.get_dictionary_mut(root_id).map_err(|e| e.to_string())?;
    if vp_ref.is_none() {
        match catalog.get_mut(b"ViewerPreferences") {
            Ok(Object::Dictionary(d)) => d.set("Direction", dir),
            _ => {
                let mut d = Dictionary::new();
                d.set("Direction", dir.clone());
                catalog.set("ViewerPreferences", Object::Dictionary(d));
            }
        }
    }
    catalog.set("PageLayout", Object::Name(b"TwoColumnRight".to_vec()));

    doc.save(dest).map_err(|e| format!("保存失敗: {e}"))?;
    Ok(())
}

/// 上書き変換。`backup_ext` を指定すると `<元ファイル>.<拡張子>` にバックアップを作る。
pub fn convert_in_place(path: &Path, backup_ext: Option<&str>) -> Result<(), String> {
    let tmp = path.with_extension("pdf.r2l-tmp");
    convert(path, &tmp)?;
    if let Some(ext) = backup_ext {
        let mut b = path.as_os_str().to_owned();
        b.push(format!(".{ext}"));
        std::fs::copy(path, &b).map_err(|e| format!("バックアップ失敗: {e}"))?;
    }
    std::fs::rename(&tmp, path).map_err(|e| format!("上書き失敗: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::dictionary;

    fn sample(path: &Path) {
        let mut doc = Document::with_version("1.5");
        let pages = doc.new_object_id();
        let page = doc.add_object(dictionary! {"Type"=>"Page","Parent"=>pages,"MediaBox"=>vec![0.into(),0.into(),100.into(),100.into()]});
        doc.objects.insert(pages, Object::Dictionary(dictionary! {"Type"=>"Pages","Kids"=>vec![page.into()],"Count"=>1}));
        let cat = doc.add_object(dictionary! {"Type"=>"Catalog","Pages"=>pages});
        doc.trailer.set("Root", cat);
        doc.save(path).unwrap();
    }

    fn check(path: &Path) {
        let doc = Document::load(path).unwrap();
        let root = doc.trailer.get(b"Root").unwrap().as_reference().unwrap();
        let cat = doc.get_dictionary(root).unwrap();
        let vp = cat.get(b"ViewerPreferences").unwrap().as_dict().unwrap();
        assert_eq!(vp.get(b"Direction").unwrap().as_name().unwrap(), b"R2L");
        assert_eq!(cat.get(b"PageLayout").unwrap().as_name().unwrap(), b"TwoColumnRight");
    }

    #[test]
    fn converts_and_in_place() {
        let d = std::env::temp_dir().join("pdfr2l_script_test");
        std::fs::create_dir_all(&d).unwrap();
        let (a, b) = (d.join("a.pdf"), d.join("b.pdf"));
        sample(&a);
        convert(&a, &b).unwrap();
        check(&b);
        assert!(convert(&a, &a).is_err());
        convert_in_place(&a, Some("bak")).unwrap();
        check(&a);
        assert!(d.join("a.pdf.bak").exists());
    }
}
